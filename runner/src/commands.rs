//! The commands behind the CLI.
//!
//! Everything a learner reads is assembled from `strings_de`. The `intern`
//! commands are developer/author tooling and speak English on purpose — they
//! run in CI, not in a classroom.

use std::io::Write;
use std::path::Path;

use serde_json::json;

use crate::capture;
use crate::checks::{self, ExerciseResult};
use crate::cli::Intern;
use crate::content;
use crate::error::{AppError, Result};
use crate::exercise::{self, Exercise, Level, EXERCISES_DIR};
use crate::progress::{Progress, STATUS_PASSED};
use crate::report;
use crate::strings_de::{self as de, Symbols};
use crate::workspace::Workspace;

// ---------------------------------------------------------------------------
// hilfe / loesung / deo-gratias
// ---------------------------------------------------------------------------

pub fn hilfe() -> Result<i32> {
    println!("{}", de::hilfe());
    Ok(0)
}

pub fn loesung(workspace: &Workspace, id: &str) -> Result<i32> {
    // A typo must not turn into a wall of red text; fall back to what was typed.
    let shown = workspace
        .resolve(id)
        .map(|exercise| exercise.id.clone())
        .unwrap_or_else(|_| id.to_string());
    println!("{}", de::loesung(&shown));
    Ok(0)
}

pub fn deo_gratias() -> Result<i32> {
    println!("{}", de::deo_gratias());
    Ok(0)
}

// ---------------------------------------------------------------------------
// check
// ---------------------------------------------------------------------------

pub fn check(
    workspace: &Workspace,
    id: Option<&str>,
    json_output: bool,
    symbols: Symbols,
) -> Result<i32> {
    if workspace.exercises.is_empty() {
        return Err(AppError::new(de::keine_uebungen(EXERCISES_DIR)));
    }
    let mut progress = Progress::load(&workspace.progress_path());

    let selected = match id {
        Some(needle) => Some(workspace.resolve(needle)?),
        None => first_open(workspace, &progress),
    };

    let Some(exercise) = selected else {
        // Nothing left to do — the learner is done.
        if json_output {
            println!(
                "{}",
                json!({ "befehl": "check", "alles_bestanden": true, "uebung": null })
            );
        } else {
            println!("{}", de::alles_geschafft());
        }
        return Ok(0);
    };

    let result = checks::run_all(exercise);
    progress.record(exercise, &result, true);
    progress.save(&workspace.progress_path())?;

    let passed = result.is_passed(exercise);
    if json_output {
        println!("{}", check_json(exercise, &result));
    } else {
        print_check_text(workspace, &progress, exercise, &result, symbols);
    }
    Ok(if passed { 0 } else { 1 })
}

fn print_check_text(
    workspace: &Workspace,
    progress: &Progress,
    exercise: &Exercise,
    result: &ExerciseResult,
    symbols: Symbols,
) {
    println!();
    println!("{}", de::uebung_kopf(&exercise.id, &exercise.title));
    println!(
        "{}",
        de::uebung_meta(exercise.minutes, exercise.difficulty, exercise.ai_level)
    );
    if exercise.lb_relevant {
        println!("{}", de::LB_HINWEIS);
    }

    for level in Level::ALL {
        let tally = result.tally(exercise, level);
        if tally.total == 0 {
            continue;
        }
        println!();
        println!("{}", de::stufen_titel(level));
        for (check, outcome) in exercise.checks.iter().zip(&result.outcomes) {
            if check.level != level {
                continue;
            }
            println!("{}", de::check_zeile(&symbols, outcome.passed, &check.id));
            if !outcome.passed {
                println!("{}", de::hinweis_zeile(&check.hint_de));
                if let Some(detail) = &outcome.detail {
                    println!("{}", de::detail_zeile(detail));
                }
            }
        }
        println!("  {}", de::tally_zeile(level, tally));
    }

    if !exercise.deepening.is_empty() {
        println!();
        println!("{}", de::vertiefung(&exercise.deepening));
    }

    println!();
    if result.is_passed(exercise) {
        println!("{}", de::check_geschafft(&exercise.id));
        let extras_open = Level::ALL
            .iter()
            .filter(|level| **level != Level::Basis)
            .any(|level| !result.tally(exercise, *level).is_complete());
        if extras_open {
            println!("{}", de::bonus_offen());
        }
        match next_open_after(workspace, progress, &exercise.id) {
            Some(next) => println!("{}", de::naechster_schritt_check(&next.id)),
            None => println!("{}", de::alles_geschafft()),
        }
    } else {
        println!("{}", de::check_offen());
        println!("{}", de::nochmal_pruefen(&exercise.id));
    }
    println!();
}

fn check_json(exercise: &Exercise, result: &ExerciseResult) -> String {
    let checks: Vec<serde_json::Value> = exercise
        .checks
        .iter()
        .zip(&result.outcomes)
        .map(|(check, outcome)| {
            json!({
                "id": check.id,
                "typ": check.kind.type_name(),
                "stufe": check.level.as_str(),
                "erfuellt": outcome.passed,
                "hinweis": if outcome.passed { None } else { Some(check.hint_de.clone()) },
                "detail": outcome.detail.as_ref().map(de::detail_text),
            })
        })
        .collect();

    let stufen: serde_json::Map<String, serde_json::Value> = Level::ALL
        .iter()
        .map(|level| {
            let tally = result.tally(exercise, *level);
            (
                level.as_str().to_string(),
                json!({ "erfuellt": tally.passed, "gesamt": tally.total }),
            )
        })
        .collect();

    json!({
        "befehl": "check",
        "uebung": {
            "id": exercise.id,
            "titel": exercise.title,
            "modul": exercise.module,
            "lb_relevant": exercise.lb_relevant,
            "ki_stufe": exercise.ai_level.as_str(),
            "schwierigkeit": exercise.difficulty,
            "zeit_minuten": exercise.minutes,
        },
        "bestanden": result.is_passed(exercise),
        "checks": checks,
        "stufen": stufen,
    })
    .to_string()
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

pub fn status(workspace: &Workspace, json_output: bool, symbols: Symbols) -> Result<i32> {
    let mut progress = Progress::load(&workspace.progress_path());
    let results: Vec<(&Exercise, ExerciseResult)> = workspace
        .exercises
        .iter()
        .map(|exercise| (exercise, checks::run_all(exercise)))
        .collect();
    for (exercise, result) in &results {
        // `status` looks, it does not count as an attempt.
        progress.record(exercise, result, false);
    }
    progress.save(&workspace.progress_path())?;

    let passed_count = results
        .iter()
        .filter(|(exercise, result)| result.is_passed(exercise))
        .count();
    let next = results
        .iter()
        .find(|(exercise, result)| !result.is_passed(exercise))
        .map(|(exercise, _)| *exercise);

    if json_output {
        println!("{}", status_json(workspace, &results, passed_count, next));
        return Ok(0);
    }

    println!();
    println!("{}", de::status_kopf(&workspace.module_name()));
    println!();
    let id_width = results
        .iter()
        .map(|(exercise, _)| exercise.id.chars().count())
        .max()
        .unwrap_or(2);
    let title_width = results
        .iter()
        .map(|(exercise, _)| exercise.title.chars().count())
        .max()
        .unwrap_or(2);

    for (exercise, result) in &results {
        let marker = if result.is_passed(exercise) {
            symbols.done
        } else if result.is_started() {
            symbols.started
        } else {
            symbols.open
        };
        let lb = if exercise.lb_relevant {
            de::STATUS_LB_MARKE
        } else {
            "  "
        };
        let stufen = Level::ALL
            .iter()
            .filter_map(|level| {
                let tally = result.tally(exercise, *level);
                (tally.total > 0).then(|| {
                    format!(
                        "{} {}/{}",
                        de::stufen_kurz(*level),
                        tally.passed,
                        tally.total
                    )
                })
            })
            .collect::<Vec<_>>()
            .join(" · ");
        println!(
            "  {}{:<id_width$}  {:<title_width$}  {lb}  {stufen}",
            symbols.cell(marker),
            exercise.id,
            exercise.title
        );
    }

    let lb_total = workspace.exercises.iter().filter(|e| e.lb_relevant).count();
    let lb_passed = results
        .iter()
        .filter(|(exercise, result)| exercise.lb_relevant && result.is_passed(exercise))
        .count();

    println!();
    println!("{}", de::status_fortschritt(passed_count, results.len()));
    if lb_total > 0 {
        println!("{}", de::status_lb_zeile(lb_passed, lb_total));
    }
    println!("{}", de::fortschrittsbalken(passed_count, results.len()));
    println!();
    match next {
        Some(exercise) => println!("{}", de::status_naechster(&exercise.id)),
        None => println!("{}", de::alles_geschafft()),
    }
    if !workspace.broken.is_empty() {
        println!();
        println!(
            "{}",
            de::status_kaputte_uebungen(
                &workspace
                    .broken
                    .iter()
                    .map(|(path, _)| path.display().to_string())
                    .collect::<Vec<_>>()
            )
        );
    }
    println!();
    Ok(0)
}

fn status_json(
    workspace: &Workspace,
    results: &[(&Exercise, ExerciseResult)],
    passed_count: usize,
    next: Option<&Exercise>,
) -> String {
    let uebungen: Vec<serde_json::Value> = results
        .iter()
        .map(|(exercise, result)| {
            let stufen: serde_json::Map<String, serde_json::Value> = Level::ALL
                .iter()
                .map(|level| {
                    let tally = result.tally(exercise, *level);
                    (
                        level.as_str().to_string(),
                        json!({ "erfuellt": tally.passed, "gesamt": tally.total }),
                    )
                })
                .collect();
            json!({
                "id": exercise.id,
                "titel": exercise.title,
                "lb_relevant": exercise.lb_relevant,
                "status": de::bericht_status(result.is_passed(exercise), result.is_started()),
                "stufen": stufen,
            })
        })
        .collect();

    json!({
        "befehl": "status",
        "modul": workspace.module_name(),
        "uebungen": uebungen,
        "bestanden": passed_count,
        "gesamt": results.len(),
        "naechste": next.map(|exercise| exercise.id.clone()),
        "kaputt": workspace
            .broken
            .iter()
            .map(|(path, problems)| json!({ "pfad": path.display().to_string(), "probleme": problems }))
            .collect::<Vec<_>>(),
    })
    .to_string()
}

// ---------------------------------------------------------------------------
// erfasse
// ---------------------------------------------------------------------------

pub fn erfasse(
    workspace: &Workspace,
    name: Option<&str>,
    id: Option<&str>,
    ordner: Option<&str>,
) -> Result<i32> {
    let Some(name) = name else {
        let rows: Vec<(String, String, bool)> = capture::PRESETS
            .iter()
            .map(|preset| {
                (
                    preset.name.to_string(),
                    preset.description_de.to_string(),
                    preset.available_here(),
                )
            })
            .collect();
        print!("{}", de::erfasse_uebersicht(&rows));
        return Ok(0);
    };

    let Some(preset) = capture::find(name) else {
        return Err(AppError::new(de::erfasse_unbekannt(
            name,
            &capture::names(),
        )));
    };
    if !preset.available_here() {
        return Err(AppError::new(de::erfasse_nur_windows(preset.name)));
    }

    let exercise = match id {
        Some(needle) => workspace.resolve(needle)?,
        None => current_or_last(workspace, &Progress::load(&workspace.progress_path()))?,
    };

    println!("{}", de::erfasse_laeuft(preset.name));
    let capture = capture::run(preset, &exercise.dir, ordner)?;

    let target = capture::target_path(preset, &exercise.dir);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            AppError::new(de::schreibfehler(
                &parent.display().to_string(),
                &e.to_string(),
            ))
        })?;
    }
    std::fs::write(&target, capture.text.as_bytes()).map_err(|e| {
        AppError::new(de::schreibfehler(
            &target.display().to_string(),
            &e.to_string(),
        ))
    })?;

    let shown = format!("{}/{}", exercise::SUBMISSION_DIR, preset.file);
    if capture.text.trim().is_empty() {
        println!("{}", de::erfasse_leer(&shown));
    } else {
        println!(
            "{}",
            de::erfasse_gespeichert(&shown, capture.text.lines().count())
        );
    }
    if capture.command_failed {
        println!("{}", de::erfasse_meldete_fehler());
    }
    println!("{}", de::nochmal_pruefen(&exercise.id));
    Ok(0)
}

// ---------------------------------------------------------------------------
// bericht
// ---------------------------------------------------------------------------

pub fn bericht(workspace: &Workspace, alias: Option<&str>) -> Result<i32> {
    if workspace.exercises.is_empty() {
        return Err(AppError::new(de::keine_uebungen(EXERCISES_DIR)));
    }
    let mut progress = Progress::load(&workspace.progress_path());
    let results: Vec<(&Exercise, ExerciseResult)> = workspace
        .exercises
        .iter()
        .map(|exercise| (exercise, checks::run_all(exercise)))
        .collect();
    for (exercise, result) in &results {
        progress.record(exercise, result, false);
    }

    let alias = match alias.map(str::trim).filter(|a| !a.is_empty()) {
        Some(given) => given.to_string(),
        None => match progress.alias.clone().filter(|a| !a.trim().is_empty()) {
            Some(stored) => stored,
            None => ask_for_alias(),
        },
    };
    progress.alias = Some(alias.clone());
    progress.save(&workspace.progress_path())?;

    let report = report::build(&alias, &workspace.module_name(), &results, &progress);
    let written = report::write(&workspace.root, &report)?;
    println!(
        "{}",
        de::bericht_geschrieben(
            &written.text_path.display().to_string(),
            &written.json_path.display().to_string()
        )
    );
    Ok(0)
}

/// Ask once, locally. Empty input is fine — no learner is forced to give a name.
fn ask_for_alias() -> String {
    print!("{} ", de::BERICHT_FRAGE_ALIAS);
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                de::BERICHT_OHNE_NAME.to_string()
            } else {
                trimmed.to_string()
            }
        }
        Err(_) => de::BERICHT_OHNE_NAME.to_string(),
    }
}

// ---------------------------------------------------------------------------
// intern (developer/author tooling, English output)
// ---------------------------------------------------------------------------

pub fn intern(command: &Intern, explicit_root: Option<&Path>) -> Result<i32> {
    match command {
        Intern::Lint { pfad } => lint(pfad.as_deref(), explicit_root),
        Intern::Hash { salt, antworten } => hash(salt, antworten),
    }
}

fn lint(path: Option<&Path>, explicit_root: Option<&Path>) -> Result<i32> {
    let dir = match (path, explicit_root) {
        (Some(path), _) => path.to_path_buf(),
        (None, Some(root)) => root.join(EXERCISES_DIR),
        (None, None) => Path::new(EXERCISES_DIR).to_path_buf(),
    };
    if !dir.is_dir() {
        return Err(AppError::new(format!(
            "lint: `{}` is not a directory",
            dir.display()
        )));
    }

    let dirs = exercise::discover(&dir);
    if dirs.is_empty() {
        return Err(AppError::new(format!(
            "lint: no exercise.toml found under `{}`",
            dir.display()
        )));
    }

    let mut failures = 0usize;
    let mut seen_ids: Vec<(String, std::path::PathBuf)> = Vec::new();
    for exercise_dir in &dirs {
        match exercise::load(exercise_dir) {
            Ok(loaded) => {
                if let Some((_, other)) = seen_ids.iter().find(|(id, _)| *id == loaded.id) {
                    failures += 1;
                    println!(
                        "FAIL {}\n  - duplicate exercise id `{}` (also in {})",
                        exercise_dir.display(),
                        loaded.id,
                        other.display()
                    );
                    continue;
                }
                seen_ids.push((loaded.id.clone(), exercise_dir.clone()));

                // Loadable is not the same as fit to hand to a human: an
                // exercise whose task text and checks disagree passes every
                // schema rule and still burns the learner.
                let content_problems = content::audit(&loaded);
                if !content_problems.is_empty() {
                    failures += 1;
                    println!("FAIL {}", exercise_dir.display());
                    for problem in content_problems {
                        println!("  - {problem}");
                    }
                    continue;
                }

                let basis = loaded.count_of(Level::Basis);
                let bonus = loaded.count_of(Level::Bonus);
                let homelab = loaded.count_of(Level::Homelab);
                println!(
                    "ok   {:<28} {} checks (basis {basis}, bonus {bonus}, homelab {homelab}){}",
                    loaded.id,
                    loaded.checks.len(),
                    if loaded.lb_relevant { " [LB]" } else { "" }
                );
            }
            Err(problems) => {
                failures += 1;
                println!("FAIL {}", exercise_dir.display());
                for problem in problems {
                    println!("  - {problem}");
                }
            }
        }
    }

    println!();
    if failures == 0 {
        println!("{} exercise(s) checked, all valid.", dirs.len());
        Ok(0)
    } else {
        println!("{} of {} exercise(s) invalid.", failures, dirs.len());
        Ok(2)
    }
}

/// Authoring helper: answers in, `expect_hash` entries out.
///
/// Only hashes reach stdout — the plaintext answers must never end up in this
/// repository (CLAUDE.md rule 6).
fn hash(salt: &str, answers: &[String]) -> Result<i32> {
    if salt.trim().is_empty() {
        return Err(AppError::new("hash: --salt must not be empty"));
    }
    println!("expect_hash = [");
    for answer in answers {
        println!("  \"{}\",", checks::answers::hash(answer, salt));
    }
    println!("]");
    eprintln!("(salt: {salt} — keep the plaintext answers in the private solutions repo)");
    Ok(0)
}

// ---------------------------------------------------------------------------
// shared helpers
// ---------------------------------------------------------------------------

/// The exercise a bare `wb check` means: the first one not yet *recorded* as
/// passed.
///
/// Recorded, not recomputed, on purpose. A learner who just fixed exercise 01
/// and types `wb check` must see "geschafft" for the work they just did — not
/// be jumped to exercise 02 and shown a fresh wall of open checks. The next
/// `wb check` then moves on, because 01 is recorded as passed by now.
fn first_open<'a>(workspace: &'a Workspace, progress: &Progress) -> Option<&'a Exercise> {
    workspace.exercises.iter().find(|exercise| {
        progress
            .get(&exercise.id)
            .map(|entry| entry.status != STATUS_PASSED)
            .unwrap_or(true)
    })
}

/// Where `wb erfasse` writes when the learner did not name an exercise.
fn current_or_last<'a>(workspace: &'a Workspace, progress: &Progress) -> Result<&'a Exercise> {
    if let Some(exercise) = first_open(workspace, progress) {
        return Ok(exercise);
    }
    workspace
        .exercises
        .last()
        .ok_or_else(|| AppError::new(de::keine_uebungen(EXERCISES_DIR)))
}

/// The exercise to suggest after `current` was passed: the next open one in
/// reading order, wrapping around to earlier ones that were skipped.
fn next_open_after<'a>(
    workspace: &'a Workspace,
    progress: &Progress,
    current: &str,
) -> Option<&'a Exercise> {
    let position = workspace.exercises.iter().position(|e| e.id == current)?;
    let count = workspace.exercises.len();
    (1..count)
        .map(|offset| &workspace.exercises[(position + offset) % count])
        .find(|exercise| {
            progress
                .get(&exercise.id)
                .map(|entry| entry.status != STATUS_PASSED)
                .unwrap_or(true)
        })
}
