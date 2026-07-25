//! `wb bericht` — the hand-in summary.
//!
//! The learner hands this file over themselves; nothing is ever transmitted
//! (PRD §7.6). The integrity hash makes casual editing visible and is
//! documented as exactly that — not an exam-grade guarantee (SPEC §7).

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::checks::ExerciseResult;
use crate::clock;
use crate::error::{AppError, Result};
use crate::exercise::{Exercise, Level};
use crate::progress::Progress;
use crate::strings_de as de;

/// Compiled-in salt for the report checksum. Not a secret, and not meant to be
/// one — it only makes "open the file and edit a line" detectable.
const REPORT_SALT: &str = "werkbank-bericht-v1";

#[derive(Debug, Serialize)]
pub struct ReportExercise {
    pub id: String,
    pub titel: String,
    pub lb_relevant: bool,
    pub status: String,
    pub versuche: u32,
    pub basis_erfuellt: u32,
    pub basis_gesamt: u32,
    pub bonus_erfuellt: u32,
    pub bonus_gesamt: u32,
    pub homelab_erfuellt: u32,
    pub homelab_gesamt: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zuletzt_geprueft: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bestanden_am: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub version: u32,
    pub alias: String,
    pub modul: String,
    pub erstellt_am: String,
    pub uebungen: Vec<ReportExercise>,
    pub bestanden: usize,
    pub gesamt: usize,
    pub lb_bestanden: usize,
    pub lb_gesamt: usize,
    pub pruefsumme: String,
}

pub fn build(
    alias: &str,
    modul: &str,
    entries: &[(&Exercise, ExerciseResult)],
    progress: &Progress,
) -> Report {
    let mut uebungen = Vec::new();
    let (mut bestanden, mut lb_bestanden, mut lb_gesamt) = (0usize, 0usize, 0usize);

    for (exercise, result) in entries {
        let stored = progress.get(&exercise.id).cloned().unwrap_or_default();
        let passed = result.is_passed(exercise);
        if passed {
            bestanden += 1;
        }
        if exercise.lb_relevant {
            lb_gesamt += 1;
            if passed {
                lb_bestanden += 1;
            }
        }
        let basis = result.tally(exercise, Level::Basis);
        let bonus = result.tally(exercise, Level::Bonus);
        let homelab = result.tally(exercise, Level::Homelab);
        uebungen.push(ReportExercise {
            id: exercise.id.clone(),
            titel: exercise.title.clone(),
            lb_relevant: exercise.lb_relevant,
            status: de::bericht_status(passed, result.is_started()).to_string(),
            versuche: stored.versuche,
            basis_erfuellt: basis.passed,
            basis_gesamt: basis.total,
            bonus_erfuellt: bonus.passed,
            bonus_gesamt: bonus.total,
            homelab_erfuellt: homelab.passed,
            homelab_gesamt: homelab.total,
            zuletzt_geprueft: stored.zuletzt_geprueft.clone(),
            bestanden_am: stored.bestanden_am.clone(),
        });
    }

    let mut report = Report {
        version: 1,
        alias: alias.to_string(),
        modul: modul.to_string(),
        erstellt_am: clock::now_iso8601(),
        gesamt: uebungen.len(),
        uebungen,
        bestanden,
        lb_bestanden,
        lb_gesamt,
        pruefsumme: String::new(),
    };
    report.pruefsumme = checksum(&report);
    report
}

/// SHA-256 over a canonical rendering of the report plus a compiled-in salt.
fn checksum(report: &Report) -> String {
    let mut canonical = String::new();
    canonical.push_str(&report.alias);
    canonical.push('|');
    canonical.push_str(&report.modul);
    canonical.push('|');
    canonical.push_str(&report.erstellt_am);
    for exercise in &report.uebungen {
        canonical.push_str(&format!(
            "|{}:{}:{}:{}/{}:{}/{}:{}/{}",
            exercise.id,
            exercise.status,
            exercise.versuche,
            exercise.basis_erfuellt,
            exercise.basis_gesamt,
            exercise.bonus_erfuellt,
            exercise.bonus_gesamt,
            exercise.homelab_erfuellt,
            exercise.homelab_gesamt,
        ));
    }
    canonical.push('|');
    canonical.push_str(REPORT_SALT);
    crate::checks::answers::sha256_hex(canonical.as_bytes())
}

pub fn render_text(report: &Report) -> String {
    let mut out = String::new();
    out.push_str(de::BERICHT_TITEL);
    out.push('\n');
    out.push_str(&"=".repeat(de::BERICHT_TITEL.chars().count()));
    out.push_str("\n\n");
    out.push_str(&format!(
        "{:<12}: {}\n{:<12}: {}\n{:<12}: {}\n\n",
        de::BERICHT_FELD_NAME,
        report.alias,
        de::BERICHT_FELD_MODUL,
        report.modul,
        de::BERICHT_FELD_ERSTELLT,
        report.erstellt_am
    ));

    out.push_str(de::BERICHT_ABSCHNITT_UEBUNGEN);
    out.push('\n');
    out.push_str(&"-".repeat(de::BERICHT_ABSCHNITT_UEBUNGEN.chars().count()));
    out.push('\n');

    let id_width = report
        .uebungen
        .iter()
        .map(|e| e.id.chars().count())
        .max()
        .unwrap_or(2);
    let title_width = report
        .uebungen
        .iter()
        .map(|e| e.titel.chars().count())
        .max()
        .unwrap_or(2);
    for exercise in &report.uebungen {
        let lb = if exercise.lb_relevant { "LB" } else { "  " };
        out.push_str(&format!(
            "{:<id_width$}  {:<title_width$}  {lb}  {:<10}  Basis {}/{}  Bonus {}/{}  Homelab {}/{}  {}\n",
            exercise.id,
            exercise.titel,
            exercise.status,
            exercise.basis_erfuellt,
            exercise.basis_gesamt,
            exercise.bonus_erfuellt,
            exercise.bonus_gesamt,
            exercise.homelab_erfuellt,
            exercise.homelab_gesamt,
            de::versuche(exercise.versuche),
        ));
    }

    out.push('\n');
    out.push_str(de::BERICHT_ABSCHNITT_ZUSAMMENFASSUNG);
    out.push('\n');
    out.push_str(&"-".repeat(de::BERICHT_ABSCHNITT_ZUSAMMENFASSUNG.chars().count()));
    out.push('\n');
    out.push_str(&de::status_fortschritt(report.bestanden, report.gesamt));
    out.push('\n');
    if report.lb_gesamt > 0 {
        out.push_str(&de::status_lb_zeile(report.lb_bestanden, report.lb_gesamt));
        out.push('\n');
    }
    out.push_str(&de::fortschrittsbalken(report.bestanden, report.gesamt));
    out.push_str("\n\n");
    out.push_str(&format!("Prüfsumme: {}\n", report.pruefsumme));
    out.push_str(de::BERICHT_PRUEFSUMME_HINWEIS);
    out.push('\n');
    out
}

pub struct WrittenReport {
    pub text_path: PathBuf,
    pub json_path: PathBuf,
}

pub fn write(root: &Path, report: &Report) -> Result<WrittenReport> {
    let text_path = root.join("bericht.txt");
    let json_path = root.join("bericht.json");
    std::fs::write(&text_path, render_text(report)).map_err(|e| {
        AppError::new(de::schreibfehler(
            &text_path.display().to_string(),
            &e.to_string(),
        ))
    })?;
    let json = serde_json::to_string_pretty(report).map_err(|e| {
        AppError::new(de::schreibfehler(
            &json_path.display().to_string(),
            &e.to_string(),
        ))
    })?;
    std::fs::write(&json_path, json).map_err(|e| {
        AppError::new(de::schreibfehler(
            &json_path.display().to_string(),
            &e.to_string(),
        ))
    })?;
    Ok(WrittenReport {
        text_path,
        json_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Report {
        Report {
            version: 1,
            alias: "Testperson".into(),
            modul: "demo".into(),
            erstellt_am: "2026-07-25T10:00:00Z".into(),
            uebungen: vec![ReportExercise {
                id: "01-erste-schritte".into(),
                titel: "Deine erste Notiz".into(),
                lb_relevant: true,
                status: "bestanden".into(),
                versuche: 2,
                basis_erfuellt: 2,
                basis_gesamt: 2,
                bonus_erfuellt: 0,
                bonus_gesamt: 1,
                homelab_erfuellt: 0,
                homelab_gesamt: 0,
                zuletzt_geprueft: Some("2026-07-25T09:59:00Z".into()),
                bestanden_am: Some("2026-07-25T09:59:00Z".into()),
            }],
            bestanden: 1,
            gesamt: 1,
            lb_bestanden: 1,
            lb_gesamt: 1,
            pruefsumme: String::new(),
        }
    }

    #[test]
    fn checksum_is_stable_and_notices_edits() {
        let mut report = sample();
        let original = checksum(&report);
        assert_eq!(original.len(), 64);
        assert_eq!(original, checksum(&sample()), "must be deterministic");

        report.uebungen[0].status = "offen".into();
        assert_ne!(original, checksum(&report), "editing a status must show up");
    }

    #[test]
    fn text_report_is_german_and_mentions_its_limits() {
        let mut report = sample();
        report.pruefsumme = checksum(&report);
        let text = render_text(&report);
        assert!(text.contains("WERKBANK — BERICHT"));
        assert!(text.contains("Übungen"));
        assert!(text.contains("bestanden"));
        assert!(text.contains("Prüfsumme:"));
        assert!(text.contains("kein Schutz gegen absichtliche Manipulation"));
        // no English leaking into a learner/trainer facing document
        for word in ["passed", "failed", "Exercise", "Summary"] {
            assert!(!text.contains(word), "English word `{word}` in bericht.txt");
        }
    }
}
