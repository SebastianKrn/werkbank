//! `abgabe/antworten.toml`: reading, normalising and hashing learner answers.
//!
//! Expected answers exist only as salted SHA-256 hashes in the exercise (rule 6
//! in CLAUDE.md). This module is the only place that knows how an answer is
//! turned into a hash — the same code path is used by `wb intern hash` so
//! authoring and checking can never drift apart.

use std::collections::BTreeMap;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::text;

#[derive(Debug)]
pub enum AnswersError {
    /// File is not there yet — the normal "not started" case.
    Missing,
    /// File exists but TOML does not parse.
    Parse(String),
    /// A value shape we cannot compare (array/table).
    UnsupportedValue { key: String },
}

#[derive(Debug, Default, Clone)]
pub struct Answers {
    values: BTreeMap<String, String>,
    lowercase_index: BTreeMap<String, String>,
}

impl Answers {
    /// Look up a key: exact first, then case-insensitive so that `Speichertyp`
    /// works where the exercise asked for `speichertyp`.
    pub fn get(&self, key: &str) -> Option<&str> {
        if let Some(value) = self.values.get(key) {
            return Some(value.as_str());
        }
        self.lowercase_index
            .get(&key.to_lowercase())
            .map(|s| s.as_str())
    }

    fn insert(&mut self, key: String, value: String) {
        self.lowercase_index
            .insert(key.to_lowercase(), value.clone());
        self.values.insert(key, value);
    }
}

/// Parse an answers file. Values may be strings, numbers or booleans — a
/// beginner writing `kerne = 4` must not get a cryptic error.
pub fn parse(text: &str) -> Result<Answers, AnswersError> {
    let table: toml::Table = text
        .parse()
        .map_err(|e: toml::de::Error| AnswersError::Parse(e.to_string()))?;
    let mut answers = Answers::default();
    for (key, value) in table {
        let rendered = match value {
            toml::Value::String(s) => s,
            toml::Value::Integer(i) => i.to_string(),
            toml::Value::Float(f) => f.to_string(),
            toml::Value::Boolean(b) => b.to_string(),
            toml::Value::Datetime(d) => d.to_string(),
            toml::Value::Array(_) | toml::Value::Table(_) => {
                return Err(AnswersError::UnsupportedValue { key })
            }
        };
        answers.insert(key, rendered);
    }
    Ok(answers)
}

/// Read an answers file from disk, tolerating Windows encodings.
pub fn load(path: &Path) -> Result<Answers, AnswersError> {
    if !path.is_file() {
        return Err(AnswersError::Missing);
    }
    let (text, _encoding) = text::read(path).map_err(|e| AnswersError::Parse(e.to_string()))?;
    parse(&text)
}

/// trim -> lowercase -> collapse inner whitespace (SPEC §3).
pub fn normalize(answer: &str) -> String {
    let lowered = answer.trim().to_lowercase();
    lowered.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `sha256(normalize(answer) + salt)`, lowercase hex.
pub fn hash(answer: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(normalize(answer).as_bytes());
    hasher.update(salt.as_bytes());
    hex(&hasher.finalize())
}

/// Plain SHA-256 over bytes, lowercase hex (used for report integrity).
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_rules() {
        assert_eq!(normalize("  SSD  "), "ssd");
        assert_eq!(normalize("Windows   Server\t2022"), "windows server 2022");
        assert_eq!(normalize("\nGPT\r\n"), "gpt");
        assert_eq!(normalize("RAID 1"), "raid 1");
        // German umlauts lowercase correctly and are not stripped
        assert_eq!(normalize("GRÖSSE Ä"), "grösse ä");
        assert_eq!(normalize("Größe"), "größe");
        // "SS" and "ß" stay different strings — content authors therefore ship
        // one expect_hash per accepted spelling.
        assert_ne!(normalize("GROSSE"), normalize("große"));
    }

    #[test]
    fn hash_is_stable_and_salted() {
        let a = hash("SSD", "wb1:01");
        let b = hash(" ssd ", "wb1:01");
        let c = hash("SSD", "wb1:02");
        assert_eq!(a, b, "normalisation must make these identical");
        assert_ne!(a, c, "different salt must give a different hash");
        assert_eq!(a.len(), 64);
        assert!(a
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }

    #[test]
    fn parses_common_value_shapes() {
        let answers = parse(
            r#"
            speichertyp = "SSD"
            kerne = 4
            uefi = true
            groesse = 1.5
            "#,
        )
        .unwrap();
        assert_eq!(answers.get("speichertyp"), Some("SSD"));
        assert_eq!(answers.get("kerne"), Some("4"));
        assert_eq!(answers.get("uefi"), Some("true"));
        assert_eq!(answers.get("groesse"), Some("1.5"));
    }

    #[test]
    fn key_lookup_is_case_insensitive() {
        let answers = parse(r#"Speichertyp = "SSD""#).unwrap();
        assert_eq!(answers.get("speichertyp"), Some("SSD"));
        assert_eq!(answers.get("Speichertyp"), Some("SSD"));
        assert_eq!(answers.get("fehlt"), None);
    }

    #[test]
    fn rejects_unsupported_values() {
        let err = parse(r#"liste = ["a", "b"]"#).unwrap_err();
        assert!(matches!(err, AnswersError::UnsupportedValue { .. }));
    }

    #[test]
    fn reports_broken_toml() {
        let err = parse("das ist kein toml").unwrap_err();
        assert!(matches!(err, AnswersError::Parse(_)));
    }
}
