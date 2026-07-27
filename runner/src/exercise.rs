//! The exercise format: `exercise.toml` parsing, validation and discovery.
//!
//! This module is deliberately strict. Content is code (CLAUDE.md, engineering
//! standards): a typo in an exercise must fail in CI, not in front of a nervous
//! beginner. Every error message here is developer/author facing (English) —
//! learner-facing wrapping happens in `strings_de`.

use std::collections::BTreeSet;
use std::fmt;
use std::path::{Path, PathBuf};

use regex::{Regex, RegexBuilder};
use serde::Deserialize;

pub const EXERCISE_FILE: &str = "exercise.toml";
pub const EXERCISES_DIR: &str = "uebungen";
pub const SUBMISSION_DIR: &str = "abgabe";
pub const DEFAULT_ANSWERS_FILE: &str = "abgabe/antworten.toml";

/// Upper bound for a compiled check regex. Content is trusted-ish (we ship it),
/// but a runaway pattern should fail lint, not eat the learner's RAM.
const REGEX_SIZE_LIMIT: usize = 1 << 20;

// ---------------------------------------------------------------------------
// Validated model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Basis,
    Bonus,
    Homelab,
}

impl Level {
    pub const ALL: [Level; 3] = [Level::Basis, Level::Bonus, Level::Homelab];

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Basis => "basis",
            Level::Bonus => "bonus",
            Level::Homelab => "homelab",
        }
    }

    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "basis" => Ok(Level::Basis),
            "bonus" => Ok(Level::Bonus),
            "homelab" => Ok(Level::Homelab),
            other => Err(format!(
                "unknown stufe `{other}` (allowed: basis, bonus, homelab)"
            )),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiLevel {
    /// Solve it yourself, no AI.
    Ohne,
    /// Solve it yourself first, then compare with AI.
    Danach,
    /// AI use is free.
    Frei,
}

impl AiLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            AiLevel::Ohne => "ohne",
            AiLevel::Danach => "danach",
            AiLevel::Frei => "frei",
        }
    }

    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "ohne" => Ok(AiLevel::Ohne),
            "danach" => Ok(AiLevel::Danach),
            "frei" => Ok(AiLevel::Frei),
            other => Err(format!(
                "exercise.ki_stufe `{other}` is not allowed (allowed: ohne, danach, frei)"
            )),
        }
    }
}

/// A path taken from exercise content. Guaranteed relative and free of `..`.
///
/// Binding security constraint (SPEC §2): paths from `exercise.toml` must
/// resolve inside the exercise folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelPath(String);

impl RelPath {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err("path is empty".to_string());
        }
        if trimmed.contains('\0') {
            return Err("path contains a NUL byte".to_string());
        }
        let unified = trimmed.replace('\\', "/");
        if unified.starts_with('/') {
            return Err(format!("path `{raw}` must be relative, not absolute"));
        }
        let bytes = unified.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            return Err(format!("path `{raw}` must not contain a drive letter"));
        }
        if unified.contains(':') {
            return Err(format!(
                "path `{raw}` must not contain `:` (NTFS alternate data stream)"
            ));
        }
        for segment in unified.split('/') {
            match segment {
                "" => return Err(format!("path `{raw}` contains an empty segment")),
                "." | ".." => {
                    return Err(format!(
                        "path `{raw}` must not contain `.` or `..` segments"
                    ))
                }
                _ => {}
            }
            if is_windows_device_name(segment) {
                return Err(format!(
                    "path `{raw}` uses the reserved Windows device name `{segment}`"
                ));
            }
        }
        Ok(RelPath(unified))
    }

    /// Join onto the exercise directory. Windows accepts `/` separators, so the
    /// stored forward-slash form works on every target.
    pub fn to_path(&self, exercise_dir: &Path) -> PathBuf {
        exercise_dir.join(&self.0)
    }
}

/// Windows resolves these names as devices in *any* directory, with or without
/// an extension: reading `abgabe/CON` blocks until the console sends EOF, which
/// would hang `wb check` with no diagnostic. Rejecting them at parse time keeps
/// the failure where a content author can see it.
fn is_windows_device_name(segment: &str) -> bool {
    const RESERVED: [&str; 6] = ["CON", "PRN", "AUX", "NUL", "COM", "LPT"];
    let stem = segment.split('.').next().unwrap_or(segment);
    let upper = stem.to_ascii_uppercase();
    RESERVED.iter().any(|reserved| {
        if matches!(*reserved, "COM" | "LPT") {
            // COM1..COM9 and LPT1..LPT9 only — `com.txt` is an ordinary file.
            upper.len() == 4
                && upper.starts_with(reserved)
                && upper.as_bytes()[3].is_ascii_digit()
                && upper.as_bytes()[3] != b'0'
        } else {
            upper == *reserved
        }
    })
}

impl fmt::Display for RelPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The six MVP check types (SPEC §3). Resist adding a seventh.
#[derive(Debug, Clone)]
pub enum CheckKind {
    FileExists {
        path: RelPath,
    },
    FileMatches {
        path: RelPath,
        pattern: Regex,
    },
    Answer {
        file: RelPath,
        key: String,
        salt: String,
        expect_hash: Vec<String>,
    },
    MinLines {
        path: RelPath,
        min: u32,
    },
    AllAnswers {
        file: RelPath,
        keys: Vec<String>,
    },
    ValuesEqual {
        file: RelPath,
        key_a: String,
        key_b: String,
    },
}

impl CheckKind {
    pub fn type_name(&self) -> &'static str {
        match self {
            CheckKind::FileExists { .. } => "file_exists",
            CheckKind::FileMatches { .. } => "file_matches",
            CheckKind::Answer { .. } => "antwort",
            CheckKind::MinLines { .. } => "datei_zeilen_min",
            CheckKind::AllAnswers { .. } => "alle_antworten",
            CheckKind::ValuesEqual { .. } => "werte_gleich",
        }
    }
}

pub const CHECK_TYPES: [&str; 6] = [
    "file_exists",
    "file_matches",
    "antwort",
    "datei_zeilen_min",
    "alle_antworten",
    "werte_gleich",
];

#[derive(Debug, Clone)]
pub struct Check {
    pub id: String,
    pub level: Level,
    pub hint_de: String,
    pub kind: CheckKind,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub module: String,
    pub difficulty: u8,
    pub minutes: u32,
    pub ai_level: AiLevel,
    pub lb_relevant: bool,
    pub deepening: Vec<String>,
    pub checks: Vec<Check>,
    pub dir: PathBuf,
}

impl Exercise {
    pub fn checks_of(&self, level: Level) -> impl Iterator<Item = &Check> {
        self.checks.iter().filter(move |c| c.level == level)
    }

    pub fn count_of(&self, level: Level) -> usize {
        self.checks_of(level).count()
    }
}

// ---------------------------------------------------------------------------
// Raw TOML shape
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    exercise: RawMeta,
    #[serde(default)]
    check: Vec<RawCheck>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMeta {
    id: String,
    titel: String,
    modul: String,
    schwierigkeit: u8,
    zeit_minuten: u32,
    ki_stufe: String,
    #[serde(default)]
    lb_relevant: bool,
    #[serde(default)]
    vertiefung: Vec<String>,
}

/// All check fields in one flat struct so that unknown keys are rejected and
/// wrong-field-for-this-type can be reported precisely.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCheck {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    stufe: Option<String>,
    hint_de: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    file: Option<String>,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    keys: Option<Vec<String>>,
    #[serde(default)]
    key_a: Option<String>,
    #[serde(default)]
    key_b: Option<String>,
    #[serde(default)]
    salt: Option<String>,
    #[serde(default)]
    expect_hash: Option<Vec<String>>,
    #[serde(default)]
    min: Option<u32>,
}

impl RawCheck {
    /// Fields that carry meaning for this check type. Anything else being set
    /// is an authoring mistake and must fail lint.
    fn reject_foreign_fields(&self, allowed: &[&str], errors: &mut Vec<String>) {
        let present: [(&str, bool); 9] = [
            ("path", self.path.is_some()),
            ("file", self.file.is_some()),
            ("pattern", self.pattern.is_some()),
            ("key", self.key.is_some()),
            ("keys", self.keys.is_some()),
            ("key_a", self.key_a.is_some()),
            ("key_b", self.key_b.is_some()),
            ("salt", self.salt.is_some()),
            ("expect_hash", self.expect_hash.is_some()),
        ];
        for (name, is_set) in present {
            if is_set && !allowed.contains(&name) {
                errors.push(format!(
                    "check `{}`: field `{name}` has no meaning for type `{}`",
                    self.id, self.kind
                ));
            }
        }
        if self.min.is_some() && !allowed.contains(&"min") {
            errors.push(format!(
                "check `{}`: field `min` has no meaning for type `{}`",
                self.id, self.kind
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Loading + validation
// ---------------------------------------------------------------------------

/// Load and fully validate one exercise directory.
///
/// Returns *all* problems found, not just the first — `wb intern lint` should
/// show an author everything that is wrong in one run.
pub fn load(dir: &Path) -> Result<Exercise, Vec<String>> {
    let toml_path = dir.join(EXERCISE_FILE);
    let raw_text = std::fs::read_to_string(&toml_path)
        .map_err(|e| vec![format!("cannot read {}: {e}", toml_path.display())])?;
    let raw: RawFile = toml::from_str(&raw_text)
        .map_err(|e| vec![format!("{} is not valid: {e}", toml_path.display())])?;

    let mut errors = Vec::new();
    let folder_name = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    validate_slug("exercise.id", &raw.exercise.id, &mut errors);
    if !folder_name.is_empty() && raw.exercise.id != folder_name {
        errors.push(format!(
            "exercise.id `{}` must match the folder name `{folder_name}`",
            raw.exercise.id
        ));
    }
    if raw.exercise.titel.trim().is_empty() {
        errors.push("exercise.titel is empty".to_string());
    }
    validate_slug("exercise.modul", &raw.exercise.modul, &mut errors);
    if !(1..=3).contains(&raw.exercise.schwierigkeit) {
        errors.push(format!(
            "exercise.schwierigkeit must be 1, 2 or 3 (got {})",
            raw.exercise.schwierigkeit
        ));
    }
    if raw.exercise.zeit_minuten == 0 {
        errors.push("exercise.zeit_minuten must be greater than 0".to_string());
    }
    let ai_level = match AiLevel::parse(raw.exercise.ki_stufe.trim()) {
        Ok(level) => level,
        Err(e) => {
            errors.push(e);
            AiLevel::Ohne
        }
    };
    for link in &raw.exercise.vertiefung {
        if !(link.starts_with("https://") || link.starts_with("http://")) {
            errors.push(format!(
                "exercise.vertiefung entry `{link}` must be an http(s) URL"
            ));
        }
    }

    if raw.check.is_empty() {
        errors.push("exercise has no [[check]] — a learner could never pass it".to_string());
    }

    let mut seen_ids = BTreeSet::new();
    let mut checks = Vec::new();
    for raw_check in &raw.check {
        validate_slug(
            &format!("check `{}`: id", raw_check.id),
            &raw_check.id,
            &mut errors,
        );
        if !seen_ids.insert(raw_check.id.clone()) {
            errors.push(format!("duplicate check id `{}`", raw_check.id));
        }
        if raw_check.hint_de.trim().is_empty() {
            errors.push(format!("check `{}`: hint_de is empty", raw_check.id));
        }
        let level = match raw_check.stufe.as_deref() {
            None => Level::Basis,
            Some(raw_level) => match Level::parse(raw_level.trim()) {
                Ok(level) => level,
                Err(e) => {
                    errors.push(format!("check `{}`: {e}", raw_check.id));
                    Level::Basis
                }
            },
        };
        if let Some(kind) = build_kind(raw_check, &mut errors) {
            checks.push(Check {
                id: raw_check.id.clone(),
                level,
                hint_de: raw_check.hint_de.trim().to_string(),
                kind,
            });
        }
    }

    // Only meaningful once every check actually built — otherwise a broken
    // check would also produce this as a confusing second error.
    if !raw.check.is_empty()
        && checks.len() == raw.check.len()
        && !checks.iter().any(|c| c.level == Level::Basis)
    {
        errors.push(
            "exercise has no check with stufe `basis` — it would pass without any work".to_string(),
        );
    }

    if errors.is_empty() {
        Ok(Exercise {
            id: raw.exercise.id,
            title: raw.exercise.titel.trim().to_string(),
            module: raw.exercise.modul,
            difficulty: raw.exercise.schwierigkeit,
            minutes: raw.exercise.zeit_minuten,
            ai_level,
            lb_relevant: raw.exercise.lb_relevant,
            deepening: raw.exercise.vertiefung,
            checks,
            dir: dir.to_path_buf(),
        })
    } else {
        Err(errors)
    }
}

fn build_kind(raw: &RawCheck, errors: &mut Vec<String>) -> Option<CheckKind> {
    let before = errors.len();
    let kind = match raw.kind.as_str() {
        "file_exists" => {
            raw.reject_foreign_fields(&["path"], errors);
            let path = required_path(raw, "path", raw.path.as_deref(), errors)?;
            Some(CheckKind::FileExists { path })
        }
        "file_matches" => {
            raw.reject_foreign_fields(&["path", "pattern"], errors);
            let path = required_path(raw, "path", raw.path.as_deref(), errors);
            let pattern = match raw.pattern.as_deref() {
                None => {
                    errors.push(format!("check `{}`: `pattern` is required", raw.id));
                    None
                }
                Some(pattern) => match RegexBuilder::new(pattern)
                    .size_limit(REGEX_SIZE_LIMIT)
                    .build()
                {
                    Ok(compiled) => Some(compiled),
                    Err(e) => {
                        errors.push(format!("check `{}`: pattern does not compile: {e}", raw.id));
                        None
                    }
                },
            };
            match (path, pattern) {
                (Some(path), Some(pattern)) => Some(CheckKind::FileMatches { path, pattern }),
                _ => None,
            }
        }
        "antwort" => {
            raw.reject_foreign_fields(&["file", "key", "salt", "expect_hash"], errors);
            let file = answers_file(raw, errors);
            let key = required_key(raw, "key", raw.key.as_deref(), errors);
            let salt = match raw.salt.as_deref().map(str::trim) {
                Some(salt) if !salt.is_empty() => Some(salt.to_string()),
                _ => {
                    errors.push(format!("check `{}`: `salt` is required", raw.id));
                    None
                }
            };
            let hashes = match raw.expect_hash.as_ref() {
                None => {
                    errors.push(format!("check `{}`: `expect_hash` is required", raw.id));
                    None
                }
                Some(list) if list.is_empty() => {
                    errors.push(format!("check `{}`: `expect_hash` is empty", raw.id));
                    None
                }
                Some(list) => {
                    let mut ok = true;
                    for hash in list {
                        if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
                            errors.push(format!(
                                "check `{}`: expect_hash entry `{hash}` is not a 64-character hex SHA-256",
                                raw.id
                            ));
                            ok = false;
                        } else if hash.chars().any(|c| c.is_ascii_uppercase()) {
                            errors.push(format!(
                                "check `{}`: expect_hash entry `{hash}` must be lowercase",
                                raw.id
                            ));
                            ok = false;
                        }
                    }
                    ok.then(|| list.clone())
                }
            };
            match (file, key, salt, hashes) {
                (Some(file), Some(key), Some(salt), Some(expect_hash)) => Some(CheckKind::Answer {
                    file,
                    key,
                    salt,
                    expect_hash,
                }),
                _ => None,
            }
        }
        "datei_zeilen_min" => {
            raw.reject_foreign_fields(&["path", "min"], errors);
            let path = required_path(raw, "path", raw.path.as_deref(), errors);
            let min = match raw.min {
                Some(0) | None => {
                    errors.push(format!(
                        "check `{}`: `min` is required and must be at least 1",
                        raw.id
                    ));
                    None
                }
                Some(min) => Some(min),
            };
            match (path, min) {
                (Some(path), Some(min)) => Some(CheckKind::MinLines { path, min }),
                _ => None,
            }
        }
        "alle_antworten" => {
            raw.reject_foreign_fields(&["file", "keys"], errors);
            let file = answers_file(raw, errors);
            let keys = match raw.keys.as_ref() {
                None => {
                    errors.push(format!("check `{}`: `keys` is required", raw.id));
                    None
                }
                Some(keys) if keys.is_empty() => {
                    errors.push(format!("check `{}`: `keys` is empty", raw.id));
                    None
                }
                Some(keys) => {
                    let mut ok = true;
                    for key in keys {
                        if key.trim().is_empty() {
                            errors
                                .push(format!("check `{}`: `keys` contains an empty key", raw.id));
                            ok = false;
                        }
                    }
                    ok.then(|| keys.iter().map(|k| k.trim().to_string()).collect())
                }
            };
            match (file, keys) {
                (Some(file), Some(keys)) => Some(CheckKind::AllAnswers { file, keys }),
                _ => None,
            }
        }
        "werte_gleich" => {
            raw.reject_foreign_fields(&["file", "key_a", "key_b"], errors);
            let file = answers_file(raw, errors);
            let key_a = required_key(raw, "key_a", raw.key_a.as_deref(), errors);
            let key_b = required_key(raw, "key_b", raw.key_b.as_deref(), errors);
            if let (Some(a), Some(b)) = (&key_a, &key_b) {
                if a == b {
                    errors.push(format!(
                        "check `{}`: key_a and key_b are the same key (`{a}`)",
                        raw.id
                    ));
                }
            }
            match (file, key_a, key_b) {
                (Some(file), Some(key_a), Some(key_b)) => {
                    Some(CheckKind::ValuesEqual { file, key_a, key_b })
                }
                _ => None,
            }
        }
        other => {
            errors.push(format!(
                "check `{}`: unknown type `{other}` (allowed: {})",
                raw.id,
                CHECK_TYPES.join(", ")
            ));
            None
        }
    };
    // A kind that produced errors must not be used, even if it built.
    if errors.len() > before {
        return None;
    }
    kind
}

fn answers_file(raw: &RawCheck, errors: &mut Vec<String>) -> Option<RelPath> {
    let candidate = raw.file.as_deref().unwrap_or(DEFAULT_ANSWERS_FILE);
    match RelPath::parse(candidate) {
        Ok(path) => Some(path),
        Err(e) => {
            errors.push(format!("check `{}`: {e}", raw.id));
            None
        }
    }
}

fn required_path(
    raw: &RawCheck,
    field: &str,
    value: Option<&str>,
    errors: &mut Vec<String>,
) -> Option<RelPath> {
    match value {
        None => {
            errors.push(format!("check `{}`: `{field}` is required", raw.id));
            None
        }
        Some(value) => match RelPath::parse(value) {
            Ok(path) => Some(path),
            Err(e) => {
                errors.push(format!("check `{}`: {e}", raw.id));
                None
            }
        },
    }
}

fn required_key(
    raw: &RawCheck,
    field: &str,
    value: Option<&str>,
    errors: &mut Vec<String>,
) -> Option<String> {
    match value.map(str::trim) {
        Some(key) if !key.is_empty() => Some(key.to_string()),
        _ => {
            errors.push(format!("check `{}`: `{field}` is required", raw.id));
            None
        }
    }
}

fn validate_slug(what: &str, value: &str, errors: &mut Vec<String>) {
    if value.is_empty() {
        errors.push(format!("{what} is empty"));
        return;
    }
    let valid = value.bytes().all(|b| {
        b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_' || b == b'.'
    }) && value.as_bytes()[0].is_ascii_alphanumeric();
    if !valid {
        errors.push(format!(
            "{what} `{value}` must start with a lowercase letter or digit and use only a-z, 0-9, `-`, `_`, `.`"
        ));
    }
}

// ---------------------------------------------------------------------------
// Discovery
// ---------------------------------------------------------------------------

/// Directories that never contain an exercise definition.
fn is_skipped_dir(name: &str) -> bool {
    name.starts_with('.') || matches!(name, SUBMISSION_DIR | "material" | "target" | "dist")
}

/// Find all exercise directories under `root`, sorted by path.
///
/// Supports both `uebungen/<id>/` and `uebungen/<modul>/<id>/` layouts, so a
/// ZIP can ship one module flat or several modules side by side.
pub fn discover(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    collect(root, 0, &mut found);
    found.sort();
    found
}

fn collect(dir: &Path, depth: usize, found: &mut Vec<PathBuf>) {
    if depth > 3 {
        return;
    }
    if dir.join(EXERCISE_FILE).is_file() {
        found.push(dir.to_path_buf());
        return; // exercises do not nest
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if is_skipped_dir(&name) {
            continue;
        }
        collect(&path, depth + 1, found);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_path_accepts_normal_paths() {
        assert_eq!(
            RelPath::parse("abgabe/notiz.txt").unwrap().to_string(),
            "abgabe/notiz.txt"
        );
        // backslashes from Windows-minded authors are normalised
        assert_eq!(
            RelPath::parse("abgabe\\notiz.txt").unwrap().to_string(),
            "abgabe/notiz.txt"
        );
    }

    #[test]
    fn rel_path_rejects_escapes() {
        for bad in [
            "../geheim.txt",
            "abgabe/../../etc/passwd",
            "abgabe\\..\\..\\windows",
            "/etc/passwd",
            "\\\\server\\share\\x",
            "C:/Windows/System32/config",
            "c:\\windows",
            "./abgabe/x.txt",
            "abgabe//x.txt",
            "   ",
        ] {
            assert!(
                RelPath::parse(bad).is_err(),
                "path escape was accepted: {bad}"
            );
        }
    }

    /// Windows resolves reserved device names in *every* directory, so a check
    /// on `abgabe/CON` would read the console device and block `wb check`
    /// forever. An NTFS alternate data stream (`file.txt:hidden`) is likewise
    /// something no learner can produce with the documented tooling.
    #[test]
    fn rel_path_rejects_windows_device_names_and_streams() {
        for bad in [
            "abgabe/CON",
            "abgabe/con",
            "CON",
            "abgabe/NUL.txt",
            "abgabe/COM1",
            "abgabe/lpt9.log",
            "abgabe/PRN",
            "abgabe/AUX",
            "abgabe/notiz.txt:geheim",
            "abgabe:stream/x.txt",
        ] {
            assert!(
                RelPath::parse(bad).is_err(),
                "Windows-hostile path was accepted: {bad}"
            );
        }
    }

    /// The device-name guard must not swallow ordinary learner filenames that
    /// merely start with the same letters.
    #[test]
    fn rel_path_still_accepts_ordinary_names() {
        for good in [
            "abgabe/notiz.txt",
            "abgabe/console.txt",
            "abgabe/nullwerte.csv",
            "abgabe/com.txt",
            "abgabe/auxiliar.md",
            "material/antworten-vorlage.toml",
        ] {
            assert!(
                RelPath::parse(good).is_ok(),
                "ordinary path was rejected: {good}"
            );
        }
    }

    #[test]
    fn level_defaults_and_parsing() {
        assert_eq!(Level::parse("basis").unwrap(), Level::Basis);
        assert_eq!(Level::parse("bonus").unwrap(), Level::Bonus);
        assert_eq!(Level::parse("homelab").unwrap(), Level::Homelab);
        assert!(Level::parse("Basis").is_err());
        assert!(Level::parse("extra").is_err());
    }

    #[test]
    fn ai_level_parsing() {
        assert_eq!(AiLevel::parse("ohne").unwrap(), AiLevel::Ohne);
        assert!(AiLevel::parse("vielleicht").is_err());
    }
}
