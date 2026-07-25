//! Finding the unpacked Werkbank folder and the exercises inside it.

use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::exercise::{self, Exercise, EXERCISES_DIR};
use crate::strings_de as de;

/// How far up we look for the `uebungen` folder, so `wb check` also works when
/// the learner is standing inside an exercise directory.
const MAX_UPWARDS: usize = 6;

pub struct Workspace {
    pub root: PathBuf,
    pub exercises: Vec<Exercise>,
    /// Exercises that failed to load, with their problems. Never fatal for
    /// `status` — one broken exercise must not hide the whole module.
    pub broken: Vec<(PathBuf, Vec<String>)>,
}

impl Workspace {
    pub fn open(explicit_root: Option<&Path>) -> Result<Self> {
        let root = match explicit_root {
            Some(path) => path.to_path_buf(),
            None => find_root()?,
        };
        let exercises_dir = root.join(EXERCISES_DIR);
        if !exercises_dir.is_dir() {
            return Err(AppError::new(de::kein_uebungsordner(EXERCISES_DIR)));
        }

        let mut exercises = Vec::new();
        let mut broken = Vec::new();
        for dir in exercise::discover(&exercises_dir) {
            match exercise::load(&dir) {
                Ok(loaded) => exercises.push(loaded),
                Err(problems) => broken.push((dir, problems)),
            }
        }
        exercises.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self {
            root,
            exercises,
            broken,
        })
    }

    pub fn progress_path(&self) -> PathBuf {
        self.root.join(".werkbank").join("fortschritt.json")
    }

    pub fn ids(&self) -> Vec<String> {
        self.exercises.iter().map(|e| e.id.clone()).collect()
    }

    /// Module name for the header. The pilot ships one module per ZIP; if a ZIP
    /// ever carries several, name them all.
    pub fn module_name(&self) -> String {
        let mut modules: Vec<&str> = self.exercises.iter().map(|e| e.module.as_str()).collect();
        modules.dedup();
        if modules.is_empty() {
            "—".to_string()
        } else {
            modules.join(", ")
        }
    }

    /// Resolve what the learner typed: exact id, then a unique prefix, then a
    /// unique substring ("wb check 01" or "wb check spiegel").
    pub fn resolve(&self, needle: &str) -> Result<&Exercise> {
        let needle = needle.trim().to_lowercase();
        if let Some(found) = self
            .exercises
            .iter()
            .find(|e| e.id.to_lowercase() == needle)
        {
            return Ok(found);
        }
        let prefix: Vec<&Exercise> = self
            .exercises
            .iter()
            .filter(|e| e.id.to_lowercase().starts_with(&needle))
            .collect();
        let candidates = if prefix.is_empty() {
            self.exercises
                .iter()
                .filter(|e| e.id.to_lowercase().contains(&needle))
                .collect()
        } else {
            prefix
        };
        match candidates.len() {
            1 => Ok(candidates[0]),
            0 => {
                // The exercise may exist but be unreadable — say so instead of
                // claiming it does not exist.
                if let Some((path, problems)) = self.broken.iter().find(|(path, _)| {
                    path.file_name()
                        .map(|name| name.to_string_lossy().to_lowercase().contains(&needle))
                        .unwrap_or(false)
                }) {
                    return Err(AppError::new(de::uebung_kaputt(
                        &path.display().to_string(),
                        problems,
                    )));
                }
                Err(AppError::new(de::uebung_unbekannt(&needle, &self.ids())))
            }
            _ => Err(AppError::new(de::uebung_mehrdeutig(
                &needle,
                &candidates.iter().map(|e| e.id.clone()).collect::<Vec<_>>(),
            ))),
        }
    }
}

/// Walk up from the current directory until a folder holds `uebungen`.
fn find_root() -> Result<PathBuf> {
    let start = std::env::current_dir()
        .map_err(|e| AppError::new(format!("{}\n({e})", de::kein_uebungsordner(EXERCISES_DIR))))?;
    let mut current = start.as_path();
    for _ in 0..MAX_UPWARDS {
        if current.join(EXERCISES_DIR).is_dir() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }
    Err(AppError::new(de::kein_uebungsordner(EXERCISES_DIR)))
}
