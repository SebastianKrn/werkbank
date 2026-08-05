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
//!
//! ## The second tripwire: answer lists in prose
//!
//! The paragraph above rejects the rule "no accepted answer anywhere". It does
//! not rule out a narrower one, and the gap it left was real: `AUTOREN.md`
//! illustrated a rule with `` `gpt`, `mirror`, `2`, `inkrementell` `` — four
//! live answers from four different exercises, in one table cell, in a public
//! repository.
//!
//! A single answer-shaped word in a sentence is unavoidable (`spiegel` is also
//! a capture preset, `New-Item` is a cmdlet the exercise teaches). *Several on
//! one line* is not prose, it is a list — and a list is what a reader copies.
//! Measured over every Markdown file in the repo, that rule flags exactly the
//! leak and nothing else.

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

/// Inline-code spans (`` `like this` ``) on one line, short enough to be an
/// answer rather than a command line.
fn inline_code_spans(line: &str) -> Vec<String> {
    line.split('`')
        .enumerate()
        .filter(|(position, part)| position % 2 == 1 && (1..=25).contains(&part.chars().count()))
        .map(|(_, part)| part.to_string())
        .collect()
}

/// Every Markdown file that ships in this repository.
fn markdown_files() -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![repo_root()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                // Build output and version control are not documents.
                if !matches!(name.as_str(), ".git" | "target" | "dist" | "node_modules") {
                    stack.push(path);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// A list of accepted answers is a copyable answer key, whatever the prose
/// around it claims to be doing.
#[test]
fn no_document_lists_several_accepted_answers_on_one_line() {
    let (salts, hashes) = shipped_salts_and_hashes();
    assert!(
        !salts.is_empty() && !hashes.is_empty(),
        "found no salts/hashes in uebungen/ — the tripwire would pass vacuously"
    );
    let files = markdown_files();
    assert!(!files.is_empty(), "found no Markdown to scan");

    let mut leaks = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        for (number, line) in text.lines().enumerate() {
            let found: Vec<String> = inline_code_spans(line)
                .into_iter()
                .filter(|span| salts.iter().any(|salt| hashes.contains(&hash(span, salt))))
                .collect();
            if found.len() >= 2 {
                leaks.push(format!(
                    "  {}:{} lists {} accepted answers: {found:?}",
                    path.display(),
                    number + 1,
                    found.len()
                ));
            }
        }
    }

    assert!(
        leaks.is_empty(),
        "documentation lists accepted answers together (CLAUDE.md rule 6):\n{}\n\n\
         Illustrate rules with invented words such as \"himmelblau\". One \
         answer-shaped word in a sentence is fine; a list is an answer key.",
        leaks.join("\n")
    );
}
