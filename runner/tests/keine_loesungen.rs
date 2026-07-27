//! Tripwire: no document may show a `wb intern hash` invocation whose arguments
//! are real accepted answers.
//!
//! CLAUDE.md rule 6 forbids solutions from entering this public repo, and the
//! rule drifted once already: `trainer/AUTOREN.md` documented the hashing
//! workflow using two genuine accepted answers of exercise 02, obscured with a
//! decoy salt that is worthless because the real salt sits in `exercise.toml`
//! in the same repo.
//!
//! ## Why this test is narrow
//!
//! The obvious broader test — "no accepted answer appears anywhere in docs" —
//! was tried first and rejected: it flags 67 places in `docs/` alone, including
//! `2`, `3`, `md`, `hash`, `ja`, `RAID 1` and `SHA-256`. That is not a bug in
//! the test, it is ADR 0005's premise. Answers are deliberately drawn from
//! closed domain vocabulary, so any honest prose about the module contains
//! them, and ADR 0003 already accepts that short answers are brute-forceable.
//! Plaintext-in-prose is therefore a policy matter for review, not a machine
//! check; a permanently-red test would simply be deleted.
//!
//! What *is* mechanically detectable — and what actually hands an answer over —
//! is a copyable recipe: a documented command with the plaintext in it.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// Mirrors `checks::answers::normalize` — trim, lowercase, collapse inner
/// whitespace. Reimplemented because integration tests cannot import a bin.
fn normalize(answer: &str) -> String {
    answer
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn hash(answer: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(normalize(answer).as_bytes());
    hasher.update(salt.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read_files_recursively(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        if meta.is_dir() {
            read_files_recursively(&path, out);
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("md" | "txt")
        ) {
            if let Ok(text) = std::fs::read_to_string(&path) {
                out.push((path, text));
            }
        }
    }
}

/// Every salt shipped with the content, and every hash accepted anywhere.
/// Salts are collected repo-wide on purpose: a doc that pairs a real answer
/// with a *decoy* salt is still a leak, because the real salt is public.
fn shipped_salts_and_hashes() -> (Vec<String>, HashSet<String>) {
    let mut salts = Vec::new();
    let mut hashes = HashSet::new();
    let mut stack = vec![repo_root().join("uebungen")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some("exercise.toml") {
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                for line in text.lines() {
                    if line.trim_start().starts_with("salt") {
                        if let Some(salt) = line.split('"').nth(1) {
                            salts.push(salt.to_string());
                        }
                    }
                }
                for token in text.split('"') {
                    if token.len() == 64
                        && token
                            .chars()
                            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
                    {
                        hashes.insert(token.to_string());
                    }
                }
            }
        }
    }
    salts.sort();
    salts.dedup();
    (salts, hashes)
}

/// The quoted arguments of every `intern hash` invocation in a document.
fn hash_command_arguments(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (index, _) in text.match_indices("intern hash") {
        // A documented invocation may wrap over continuation lines, so read to
        // the end of the command rather than the end of the line.
        let rest = &text[index..];
        let end = rest
            .find("\n\n")
            .or_else(|| rest.find("```"))
            .unwrap_or(rest.len());
        let command = &rest[..end];
        for (position, part) in command.split('"').enumerate() {
            // Odd indices are the quoted spans.
            if position % 2 == 1 && !part.trim().is_empty() {
                out.push(part.to_string());
            }
        }
    }
    out
}

#[test]
fn no_documented_hash_command_contains_a_real_answer() {
    let (salts, hashes) = shipped_salts_and_hashes();
    assert!(
        !salts.is_empty() && !hashes.is_empty(),
        "found no salts/hashes in uebungen/ — the tripwire would pass vacuously"
    );

    let mut prose = Vec::new();
    read_files_recursively(&repo_root().join("docs"), &mut prose);
    read_files_recursively(&repo_root().join("trainer"), &mut prose);
    prose.push((
        PathBuf::from("README.md"),
        std::fs::read_to_string(repo_root().join("README.md")).unwrap_or_default(),
    ));
    assert!(!prose.is_empty(), "found no documentation to scan");

    let mut leaks = Vec::new();
    for (path, text) in &prose {
        for argument in hash_command_arguments(text) {
            for salt in &salts {
                if hashes.contains(&hash(&argument, salt)) {
                    leaks.push(format!(
                        "  {} documents `intern hash` with the real answer {argument:?} \
                         (matches an expect_hash under salt {salt})",
                        path.display()
                    ));
                }
            }
        }
    }

    leaks.sort();
    leaks.dedup();
    assert!(
        leaks.is_empty(),
        "documented hashing examples contain real accepted answers \
         (CLAUDE.md rule 6, ADR 0003):\n{}\n\nUse an invented word such as \
         \"himmelblau\" — never a live answer, and never rely on a decoy salt: \
         the real salts are published in exercise.toml.",
        leaks.join("\n")
    );
}
