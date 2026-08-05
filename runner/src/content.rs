//! Content integrity: is this exercise fit to hand to a learner?
//!
//! `exercise::load` answers a narrower question — *can the runner run this?* It
//! deliberately says yes to an exercise whose `AUFGABE.md` is missing, because a
//! learner in the middle of the module must never be locked out by a content
//! problem they cannot fix.
//!
//! `wb intern lint` needs the wider question, and it is the one that hurts:
//! every rule below describes a mistake that lets a learner do everything right
//! and still see a red check. Those cost trust, and trust is the product.
//!
//! Author-facing English, like the rest of `intern`.

use std::path::Path;

use crate::exercise::{CheckKind, Exercise, SUBMISSION_DIR};

/// The task text a learner reads.
pub const TASK_FILE: &str = "AUFGABE.md";
/// Optional starting point for `abgabe/antworten.toml`.
pub const ANSWER_TEMPLATE: &str = "material/antworten-vorlage.toml";

/// Check the rules that only matter once a human receives the exercise.
///
/// Returns every problem found, not just the first — an author should see the
/// whole list in one run.
pub fn audit(exercise: &Exercise) -> Vec<String> {
    let mut problems = Vec::new();
    let task = read(&exercise.dir.join(TASK_FILE));
    let template = read(&exercise.dir.join(ANSWER_TEMPLATE));

    match task.as_deref() {
        None => problems.push(format!(
            "{TASK_FILE} is missing — a learner would have nothing to read"
        )),
        Some(text) if text.trim().is_empty() => problems.push(format!("{TASK_FILE} is empty")),
        Some(_) => {}
    }

    let submission_prefix = format!("{SUBMISSION_DIR}/");
    for check in &exercise.checks {
        for path in checked_paths(&check.kind) {
            if !path.starts_with(&submission_prefix) {
                problems.push(format!(
                    "check `{}`: reads `{path}`, but a check may only read what the learner \
                     writes into `{SUBMISSION_DIR}/`",
                    check.id
                ));
                continue;
            }
            // Only meaningful when there is a text to search.
            if let Some(text) = task.as_deref() {
                if !text.contains(file_name(&path)) {
                    problems.push(format!(
                        "check `{}`: `{path}` is never mentioned in {TASK_FILE} — the learner \
                         is not told to create it",
                        check.id
                    ));
                }
            }
        }

        for key in checked_keys(&check.kind) {
            let known = [task.as_deref(), template.as_deref()]
                .into_iter()
                .flatten()
                .any(|text| text.contains(&key));
            if !known {
                problems.push(format!(
                    "check `{}`: answer key `{key}` appears neither in {TASK_FILE} nor in \
                     {ANSWER_TEMPLATE} — the learner cannot know what to write",
                    check.id
                ));
            }
        }
    }

    problems
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// Files a check reads, as written in `exercise.toml`.
fn checked_paths(kind: &CheckKind) -> Vec<String> {
    match kind {
        CheckKind::FileExists { path }
        | CheckKind::FileMatches { path, .. }
        | CheckKind::MinLines { path, .. } => vec![path.to_string()],
        CheckKind::Answer { file, .. }
        | CheckKind::AllAnswers { file, .. }
        | CheckKind::ValuesEqual { file, .. } => vec![file.to_string()],
    }
}

/// Answer keys a check requires the learner to have written.
fn checked_keys(kind: &CheckKind) -> Vec<String> {
    match kind {
        CheckKind::Answer { key, .. } => vec![key.clone()],
        CheckKind::AllAnswers { keys, .. } => keys.clone(),
        CheckKind::ValuesEqual { key_a, key_b, .. } => vec![key_a.clone(), key_b.clone()],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise;

    /// Build a loadable exercise directory. `task` is written as AUFGABE.md
    /// unless it is `None`.
    fn exercise_with(
        checks: &str,
        task: Option<&str>,
        template: Option<&str>,
    ) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let exercise = dir.path().join("01-demo");
        std::fs::create_dir_all(exercise.join(SUBMISSION_DIR)).unwrap();
        std::fs::write(
            exercise.join("exercise.toml"),
            format!(
                r#"
[exercise]
id = "01-demo"
titel = "Demo"
modul = "demo"
schwierigkeit = 1
zeit_minuten = 10
ki_stufe = "ohne"
{checks}
"#
            ),
        )
        .unwrap();
        if let Some(task) = task {
            std::fs::write(exercise.join(TASK_FILE), task).unwrap();
        }
        if let Some(template) = template {
            std::fs::create_dir_all(exercise.join("material")).unwrap();
            std::fs::write(exercise.join(ANSWER_TEMPLATE), template).unwrap();
        }
        dir
    }

    fn audit_of(dir: &tempfile::TempDir) -> Vec<String> {
        let loaded = exercise::load(&dir.path().join("01-demo")).expect("fixture must load");
        audit(&loaded)
    }

    const FILE_CHECK: &str = r#"
[[check]]
id = "notiz-da"
type = "file_exists"
path = "abgabe/notiz.txt"
hint_de = "Lege abgabe/notiz.txt an."
"#;

    #[test]
    fn a_complete_exercise_has_nothing_to_report() {
        let dir = exercise_with(FILE_CHECK, Some("Lege `abgabe/notiz.txt` an."), None);
        assert_eq!(audit_of(&dir), Vec::<String>::new());
    }

    /// An exercise without task text loads fine and is useless to a human.
    #[test]
    fn a_missing_aufgabe_is_reported() {
        let dir = exercise_with(FILE_CHECK, None, None);
        let problems = audit_of(&dir);
        assert!(
            problems.iter().any(|p| p.contains("AUFGABE.md is missing")),
            "{problems:?}"
        );
    }

    #[test]
    fn an_empty_aufgabe_is_reported() {
        let dir = exercise_with(FILE_CHECK, Some("   \n\n"), None);
        let problems = audit_of(&dir);
        assert!(
            problems.iter().any(|p| p.contains("AUFGABE.md is empty")),
            "{problems:?}"
        );
    }

    /// The expensive mistake: the learner does exactly what the task says and
    /// the check still fails, because the two names drifted apart.
    #[test]
    fn a_checked_file_the_task_never_mentions_is_reported() {
        let dir = exercise_with(FILE_CHECK, Some("Lege `abgabe/notizen.txt` an."), None);
        let problems = audit_of(&dir);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("notiz.txt") && p.contains("never mentioned")),
            "{problems:?}"
        );
    }

    const ANSWER_CHECK: &str = r#"
[[check]]
id = "alle-fragen"
type = "alle_antworten"
keys = ["werkzeug"]
hint_de = "Trage werkzeug ein."
"#;

    #[test]
    fn an_answer_key_the_learner_cannot_discover_is_reported() {
        let dir = exercise_with(
            ANSWER_CHECK,
            Some("Fülle `abgabe/antworten.toml` aus."),
            None,
        );
        let problems = audit_of(&dir);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("werkzeug") && p.contains("neither")),
            "{problems:?}"
        );
    }

    /// Both content styles are legitimate: the key may be named in the task text
    /// or shipped in a template the learner copies.
    #[test]
    fn an_answer_key_named_only_in_the_template_is_accepted() {
        let dir = exercise_with(
            ANSWER_CHECK,
            Some("Kopiere `material/antworten-vorlage.toml` nach `abgabe/antworten.toml`."),
            Some("werkzeug = ''\n"),
        );
        assert_eq!(audit_of(&dir), Vec::<String>::new());
    }

    #[test]
    fn an_answer_key_named_only_in_the_task_is_accepted() {
        let dir = exercise_with(
            ANSWER_CHECK,
            Some("Trage in `abgabe/antworten.toml` den Eintrag `werkzeug` ein."),
            None,
        );
        assert_eq!(audit_of(&dir), Vec::<String>::new());
    }

    /// Checks grade the learner's own work. Reading anything else means the
    /// exercise passes on files the learner never touched.
    #[test]
    fn a_check_reading_outside_the_abgabe_folder_is_reported() {
        let checks = r#"
[[check]]
id = "vorlage-da"
type = "file_exists"
path = "material/vorlage.txt"
hint_de = "Die Vorlage muss da sein."
"#;
        let dir = exercise_with(checks, Some("Sieh dir `material/vorlage.txt` an."), None);
        let problems = audit_of(&dir);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("material/vorlage.txt") && p.contains("only read")),
            "{problems:?}"
        );
    }
}
