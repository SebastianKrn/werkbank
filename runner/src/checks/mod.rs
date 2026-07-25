//! Running the six declarative check types.
//!
//! Binding constraint (SPEC §2): nothing in here executes anything that comes
//! from exercise content. A check reads files and compares — that is all.
//!
//! German wording lives in `strings_de`; this module returns a structured
//! [`Detail`] so that every learner-facing sentence stays in one file.

pub mod answers;
pub mod cp850;
pub mod text;

use std::path::{Path, PathBuf};

use crate::exercise::{Check, CheckKind, Exercise, Level, RelPath};

/// Why a check did not pass (or what it found). Rendered by `strings_de`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Detail {
    FileMissing {
        path: String,
    },
    FileEmpty {
        path: String,
    },
    FileUnreadable {
        path: String,
        problem: String,
    },
    PatternNotFound {
        path: String,
    },
    TooFewLines {
        path: String,
        found: usize,
        min: u32,
    },
    AnswersMissing {
        path: String,
    },
    AnswersBroken {
        path: String,
        problem: String,
    },
    AnswersUnsupported {
        path: String,
        key: String,
    },
    AnswerMissing {
        path: String,
        key: String,
    },
    AnswerWrong {
        key: String,
    },
    AnswersKeysMissing {
        path: String,
        keys: Vec<String>,
    },
    ValuesDiffer {
        key_a: String,
        key_b: String,
    },
    PathEscape {
        path: String,
    },
}

#[derive(Debug, Clone)]
pub struct CheckOutcome {
    pub passed: bool,
    pub detail: Option<Detail>,
}

impl CheckOutcome {
    fn passed() -> Self {
        Self {
            passed: true,
            detail: None,
        }
    }

    fn failed(detail: Detail) -> Self {
        Self {
            passed: false,
            detail: Some(detail),
        }
    }
}

/// Counts per level, used by status, check summary and report.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tally {
    pub passed: u32,
    pub total: u32,
}

impl Tally {
    pub fn is_complete(&self) -> bool {
        self.passed >= self.total
    }
}

/// Result of running every check of one exercise.
#[derive(Debug, Clone)]
pub struct ExerciseResult {
    pub outcomes: Vec<CheckOutcome>,
}

impl ExerciseResult {
    pub fn tally(&self, exercise: &Exercise, level: Level) -> Tally {
        let mut tally = Tally::default();
        for (check, outcome) in exercise.checks.iter().zip(&self.outcomes) {
            if check.level == level {
                tally.total += 1;
                if outcome.passed {
                    tally.passed += 1;
                }
            }
        }
        tally
    }

    /// An exercise counts as passed when every `basis` check passes. Bonus and
    /// homelab checks never block (SPEC §3, differentiation mechanism).
    pub fn is_passed(&self, exercise: &Exercise) -> bool {
        self.tally(exercise, Level::Basis).is_complete()
    }

    /// Has the learner produced anything at all yet?
    pub fn is_started(&self) -> bool {
        self.outcomes.iter().any(|o| o.passed)
    }
}

/// Run all checks of an exercise, in file order.
pub fn run_all(exercise: &Exercise) -> ExerciseResult {
    let outcomes = exercise
        .checks
        .iter()
        .map(|check| run(check, &exercise.dir))
        .collect();
    ExerciseResult { outcomes }
}

/// Run a single check against an exercise directory.
pub fn run(check: &Check, exercise_dir: &Path) -> CheckOutcome {
    match &check.kind {
        CheckKind::FileExists { path } => match resolve(exercise_dir, path) {
            Err(detail) => CheckOutcome::failed(detail),
            Ok(full) => match std::fs::metadata(&full) {
                Err(_) => CheckOutcome::failed(Detail::FileMissing {
                    path: path.to_string(),
                }),
                Ok(meta) if !meta.is_file() => CheckOutcome::failed(Detail::FileMissing {
                    path: path.to_string(),
                }),
                Ok(meta) if meta.len() == 0 => CheckOutcome::failed(Detail::FileEmpty {
                    path: path.to_string(),
                }),
                Ok(_) => CheckOutcome::passed(),
            },
        },

        CheckKind::FileMatches { path, pattern } => match read_text(exercise_dir, path) {
            Err(outcome) => outcome,
            Ok(content) => {
                if pattern.is_match(&content) {
                    CheckOutcome::passed()
                } else {
                    CheckOutcome::failed(Detail::PatternNotFound {
                        path: path.to_string(),
                    })
                }
            }
        },

        CheckKind::MinLines { path, min } => match read_text(exercise_dir, path) {
            Err(outcome) => outcome,
            Ok(content) => {
                let found = text::count_non_empty_lines(&content);
                if found >= *min as usize {
                    CheckOutcome::passed()
                } else {
                    CheckOutcome::failed(Detail::TooFewLines {
                        path: path.to_string(),
                        found,
                        min: *min,
                    })
                }
            }
        },

        CheckKind::Answer {
            file,
            key,
            salt,
            expect_hash,
        } => match load_answers(exercise_dir, file) {
            Err(outcome) => outcome,
            Ok(answers) => match answers.get(key) {
                None => CheckOutcome::failed(Detail::AnswerMissing {
                    path: file.to_string(),
                    key: key.clone(),
                }),
                Some(value) if value.trim().is_empty() => {
                    CheckOutcome::failed(Detail::AnswerMissing {
                        path: file.to_string(),
                        key: key.clone(),
                    })
                }
                Some(value) => {
                    let actual = answers::hash(value, salt);
                    if expect_hash.contains(&actual) {
                        CheckOutcome::passed()
                    } else {
                        CheckOutcome::failed(Detail::AnswerWrong { key: key.clone() })
                    }
                }
            },
        },

        CheckKind::AllAnswers { file, keys } => match load_answers(exercise_dir, file) {
            Err(outcome) => outcome,
            Ok(answers) => {
                let missing: Vec<String> = keys
                    .iter()
                    .filter(|key| match answers.get(key) {
                        None => true,
                        Some(value) => value.trim().is_empty(),
                    })
                    .cloned()
                    .collect();
                if missing.is_empty() {
                    CheckOutcome::passed()
                } else {
                    CheckOutcome::failed(Detail::AnswersKeysMissing {
                        path: file.to_string(),
                        keys: missing,
                    })
                }
            }
        },

        CheckKind::ValuesEqual { file, key_a, key_b } => match load_answers(exercise_dir, file) {
            Err(outcome) => outcome,
            Ok(answers) => {
                let a = answers.get(key_a).filter(|v| !v.trim().is_empty());
                let b = answers.get(key_b).filter(|v| !v.trim().is_empty());
                match (a, b) {
                    (None, _) => CheckOutcome::failed(Detail::AnswerMissing {
                        path: file.to_string(),
                        key: key_a.clone(),
                    }),
                    (_, None) => CheckOutcome::failed(Detail::AnswerMissing {
                        path: file.to_string(),
                        key: key_b.clone(),
                    }),
                    (Some(a), Some(b)) => {
                        if answers::normalize(a) == answers::normalize(b) {
                            CheckOutcome::passed()
                        } else {
                            CheckOutcome::failed(Detail::ValuesDiffer {
                                key_a: key_a.clone(),
                                key_b: key_b.clone(),
                            })
                        }
                    }
                }
            }
        },
    }
}

/// Join a content path onto the exercise directory and make sure the result
/// really stays inside it — including via symlinks.
fn resolve(exercise_dir: &Path, rel: &RelPath) -> Result<PathBuf, Detail> {
    let joined = rel.to_path(exercise_dir);
    // Only an existing path can be canonicalised; a missing file cannot escape.
    let (Ok(real), Ok(base)) = (joined.canonicalize(), exercise_dir.canonicalize()) else {
        return Ok(joined);
    };
    if real.starts_with(&base) {
        Ok(joined)
    } else {
        Err(Detail::PathEscape {
            path: rel.to_string(),
        })
    }
}

fn read_text(exercise_dir: &Path, rel: &RelPath) -> Result<String, CheckOutcome> {
    let full = resolve(exercise_dir, rel).map_err(CheckOutcome::failed)?;
    if !full.is_file() {
        return Err(CheckOutcome::failed(Detail::FileMissing {
            path: rel.to_string(),
        }));
    }
    match text::read(&full) {
        Ok((content, _encoding)) => Ok(content),
        Err(e) => Err(CheckOutcome::failed(Detail::FileUnreadable {
            path: rel.to_string(),
            problem: e.to_string(),
        })),
    }
}

fn load_answers(exercise_dir: &Path, rel: &RelPath) -> Result<answers::Answers, CheckOutcome> {
    let full = resolve(exercise_dir, rel).map_err(CheckOutcome::failed)?;
    match answers::load(&full) {
        Ok(answers) => Ok(answers),
        Err(answers::AnswersError::Missing) => Err(CheckOutcome::failed(Detail::AnswersMissing {
            path: rel.to_string(),
        })),
        Err(answers::AnswersError::Parse(problem)) => {
            Err(CheckOutcome::failed(Detail::AnswersBroken {
                path: rel.to_string(),
                problem,
            }))
        }
        Err(answers::AnswersError::UnsupportedValue { key }) => {
            Err(CheckOutcome::failed(Detail::AnswersUnsupported {
                path: rel.to_string(),
                key,
            }))
        }
    }
}

#[cfg(test)]
mod tests;
