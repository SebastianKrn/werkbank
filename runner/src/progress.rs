//! Local progress: `.werkbank/fortschritt.json`.
//!
//! Stays on the learner's machine. No telemetry, no PII beyond the alias the
//! learner types themselves (PRD §7.6). Field names are German because this is
//! a file learners and trainers may open.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::checks::ExerciseResult;
use crate::clock;
use crate::error::{AppError, Result};
use crate::exercise::{Exercise, Level};
use crate::strings_de as de;

pub const PROGRESS_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    pub uebungen: BTreeMap<String, ExerciseProgress>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            version: PROGRESS_VERSION,
            alias: None,
            uebungen: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExerciseProgress {
    pub status: String,
    #[serde(default)]
    pub versuche: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zuletzt_geprueft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bestanden_am: Option<String>,
    #[serde(default)]
    pub basis_erfuellt: u32,
    #[serde(default)]
    pub basis_gesamt: u32,
    #[serde(default)]
    pub bonus_erfuellt: u32,
    #[serde(default)]
    pub bonus_gesamt: u32,
    #[serde(default)]
    pub homelab_erfuellt: u32,
    #[serde(default)]
    pub homelab_gesamt: u32,
}

impl ExerciseProgress {
    pub fn is_passed(&self) -> bool {
        self.status == STATUS_PASSED
    }

    pub fn is_started(&self) -> bool {
        self.status == STATUS_STARTED || self.is_passed()
    }
}

pub const STATUS_OPEN: &str = "offen";
pub const STATUS_STARTED: &str = "begonnen";
pub const STATUS_PASSED: &str = "bestanden";

impl Progress {
    /// Load, tolerating a missing or damaged file: a broken progress file must
    /// never lock a learner out of their exercises.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AppError::new(de::schreibfehler(
                    &parent.display().to_string(),
                    &e.to_string(),
                ))
            })?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            AppError::new(de::schreibfehler(
                &path.display().to_string(),
                &e.to_string(),
            ))
        })?;
        // Write next to the target and rename, so an interrupted save cannot
        // leave a half-written progress file behind.
        let temp = path.with_extension("json.tmp");
        std::fs::write(&temp, json.as_bytes()).map_err(|e| {
            AppError::new(de::schreibfehler(
                &temp.display().to_string(),
                &e.to_string(),
            ))
        })?;
        std::fs::rename(&temp, path).map_err(|e| {
            AppError::new(de::schreibfehler(
                &path.display().to_string(),
                &e.to_string(),
            ))
        })
    }

    pub fn get(&self, id: &str) -> Option<&ExerciseProgress> {
        self.uebungen.get(id)
    }

    /// Record the outcome of a run. `count_attempt` is true for `wb check`
    /// (the learner asked) and false for `wb status`/`wb bericht` (we asked).
    pub fn record(&mut self, exercise: &Exercise, result: &ExerciseResult, count_attempt: bool) {
        let passed = result.is_passed(exercise);
        let started = result.is_started();
        let now = clock::now_iso8601();
        let entry = self.uebungen.entry(exercise.id.clone()).or_default();

        if count_attempt {
            entry.versuche += 1;
            entry.zuletzt_geprueft = Some(now.clone());
        }
        entry.status = if passed {
            STATUS_PASSED
        } else if started || entry.is_started() {
            STATUS_STARTED
        } else {
            STATUS_OPEN
        }
        .to_string();
        // The date belongs to the pass. An exercise can stop passing — a
        // deleted file, an empty capture, a broken antworten.toml — and a row
        // that reads "begonnen" while carrying a pass date makes the trainer
        // doubt the whole sheet.
        match (passed, entry.bestanden_am.is_some()) {
            (true, false) => entry.bestanden_am = Some(now),
            (false, true) => entry.bestanden_am = None,
            _ => {}
        }

        let basis = result.tally(exercise, Level::Basis);
        let bonus = result.tally(exercise, Level::Bonus);
        let homelab = result.tally(exercise, Level::Homelab);
        entry.basis_erfuellt = basis.passed;
        entry.basis_gesamt = basis.total;
        entry.bonus_erfuellt = bonus.passed;
        entry.bonus_gesamt = bonus.total;
        entry.homelab_erfuellt = homelab.passed;
        entry.homelab_gesamt = homelab.total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_gives_empty_progress() {
        let progress = Progress::load(Path::new("/definitiv/nicht/da/fortschritt.json"));
        assert_eq!(progress.version, PROGRESS_VERSION);
        assert!(progress.uebungen.is_empty());
    }

    #[test]
    fn damaged_file_does_not_lock_the_learner_out() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fortschritt.json");
        std::fs::write(&path, "{ das ist kaputt").unwrap();
        let progress = Progress::load(&path);
        assert!(progress.uebungen.is_empty());
    }

    /// An exercise can stop passing: the learner deletes a file, an `erfasse`
    /// preset comes back empty, or an edit breaks `antworten.toml`. The trainer
    /// reads `bericht.txt`; a row that says "begonnen" and carries a pass date
    /// makes them doubt the sheet rather than the exercise.
    #[test]
    fn a_pass_date_does_not_outlive_the_pass() {
        let dir = tempfile::tempdir().unwrap();
        let exercise = passing_exercise(&dir);

        let mut progress = Progress::default();
        progress.record(&exercise, &all_passed(&exercise), true);
        assert!(progress.get(&exercise.id).unwrap().is_passed());
        assert!(progress.get(&exercise.id).unwrap().bestanden_am.is_some());

        progress.record(&exercise, &none_passed(&exercise), true);
        let entry = progress.get(&exercise.id).unwrap();
        assert!(!entry.is_passed(), "the exercise no longer passes");
        assert!(
            entry.bestanden_am.is_none(),
            "a pass date on a non-passing exercise contradicts its own status"
        );
    }

    fn passing_exercise(dir: &tempfile::TempDir) -> Exercise {
        let path = dir.path().join("01-test");
        std::fs::create_dir_all(path.join("abgabe")).unwrap();
        std::fs::write(
            path.join("exercise.toml"),
            r#"
[exercise]
id = "01-test"
titel = "Test"
modul = "test"
schwierigkeit = 1
zeit_minuten = 10
ki_stufe = "ohne"

[[check]]
id = "notiz-da"
type = "file_exists"
path = "abgabe/notiz.txt"
hint_de = "Lege abgabe/notiz.txt an."
"#,
        )
        .unwrap();
        crate::exercise::load(&path).expect("fixture must load")
    }

    fn all_passed(exercise: &Exercise) -> ExerciseResult {
        std::fs::write(exercise.dir.join("abgabe/notiz.txt"), "Inhalt\n").unwrap();
        crate::checks::run_all(exercise)
    }

    fn none_passed(exercise: &Exercise) -> ExerciseResult {
        std::fs::remove_file(exercise.dir.join("abgabe/notiz.txt")).unwrap();
        crate::checks::run_all(exercise)
    }

    #[test]
    fn save_and_load_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".werkbank").join("fortschritt.json");
        let mut progress = Progress {
            alias: Some("Testperson".to_string()),
            ..Default::default()
        };
        progress.uebungen.insert(
            "01-test".to_string(),
            ExerciseProgress {
                status: STATUS_PASSED.to_string(),
                versuche: 3,
                ..Default::default()
            },
        );
        progress.save(&path).unwrap();

        let loaded = Progress::load(&path);
        assert_eq!(loaded.alias.as_deref(), Some("Testperson"));
        assert!(loaded.get("01-test").unwrap().is_passed());
        assert_eq!(loaded.get("01-test").unwrap().versuche, 3);
    }
}
