//! Unit tests per check type, including the Windows encoding fallbacks and
//! path-escape attempts.
//!
//! Note on CLAUDE.md rule 6: no expected answer is stored here. Where an
//! `antwort` check is tested, the test computes the hash at runtime from its
//! own throwaway string — nothing in the repository reveals a solution to a
//! real exercise.

use std::path::{Path, PathBuf};

use super::*;
use crate::exercise;

struct Fixture {
    _dir: tempfile::TempDir,
    path: PathBuf,
}

impl Fixture {
    fn new(checks_toml: &str) -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("01-test-uebung");
        std::fs::create_dir_all(path.join("abgabe")).expect("create abgabe");
        let toml = format!(
            r#"
[exercise]
id = "01-test-uebung"
titel = "Testübung"
modul = "test"
schwierigkeit = 1
zeit_minuten = 10
ki_stufe = "ohne"

{checks_toml}
"#
        );
        std::fs::write(path.join("exercise.toml"), toml).expect("write exercise.toml");
        Self { _dir: dir, path }
    }

    fn write(&self, rel: &str, content: &str) -> &Self {
        self.write_bytes(rel, content.as_bytes())
    }

    fn write_bytes(&self, rel: &str, bytes: &[u8]) -> &Self {
        let target = self.path.join(rel);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).expect("create parent");
        }
        std::fs::write(target, bytes).expect("write file");
        self
    }

    fn dir(&self) -> &Path {
        &self.path
    }

    fn run_first(&self) -> CheckOutcome {
        let exercise = exercise::load(&self.path).expect("exercise must be valid");
        run(&exercise.checks[0], &self.path)
    }

    fn result(&self) -> (exercise::Exercise, ExerciseResult) {
        let exercise = exercise::load(&self.path).expect("exercise must be valid");
        let result = run_all(&exercise);
        (exercise, result)
    }
}

fn utf16le(text: &str, bom: bool) -> Vec<u8> {
    let mut out = Vec::new();
    if bom {
        out.extend_from_slice(&[0xFF, 0xFE]);
    }
    for unit in text.encode_utf16() {
        out.extend_from_slice(&unit.to_le_bytes());
    }
    out
}

// --- file_exists ----------------------------------------------------------

const FILE_EXISTS: &str = r#"
[[check]]
id = "datei-da"
type = "file_exists"
path = "abgabe/notiz.txt"
hint_de = "Lege die Datei abgabe/notiz.txt an."
"#;

#[test]
fn file_exists_missing() {
    let fixture = Fixture::new(FILE_EXISTS);
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::FileMissing {
            path: "abgabe/notiz.txt".into()
        })
    );
}

#[test]
fn file_exists_empty_file_does_not_count() {
    let fixture = Fixture::new(FILE_EXISTS);
    fixture.write("abgabe/notiz.txt", "");
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::FileEmpty {
            path: "abgabe/notiz.txt".into()
        })
    );
}

#[test]
fn file_exists_passes() {
    let fixture = Fixture::new(FILE_EXISTS);
    fixture.write("abgabe/notiz.txt", "Hallo");
    assert!(fixture.run_first().passed);
}

#[test]
fn file_exists_rejects_directory() {
    let fixture = Fixture::new(FILE_EXISTS);
    std::fs::create_dir_all(fixture.dir().join("abgabe/notiz.txt")).unwrap();
    assert!(!fixture.run_first().passed);
}

// --- file_matches ---------------------------------------------------------

const FILE_MATCHES: &str = r#"
[[check]]
id = "ram-erkannt"
type = "file_matches"
path = "abgabe/systeminfo.txt"
pattern = '(?i)(Gesamter physischer Speicher|Total Physical Memory)'
hint_de = "Hast du die ganze Ausgabe gespeichert?"
"#;

#[test]
fn file_matches_utf8() {
    let fixture = Fixture::new(FILE_MATCHES);
    fixture.write(
        "abgabe/systeminfo.txt",
        "Hostname: aquinas\nGesamter physischer Speicher: 16.384 MB\n",
    );
    assert!(fixture.run_first().passed);
}

#[test]
fn file_matches_utf16le_with_bom() {
    let fixture = Fixture::new(FILE_MATCHES);
    fixture.write_bytes(
        "abgabe/systeminfo.txt",
        &utf16le("Gesamter physischer Speicher: 8.192 MB\r\n", true),
    );
    assert!(
        fixture.run_first().passed,
        "PowerShell 5.1 writes UTF-16LE with BOM"
    );
}

#[test]
fn file_matches_utf16le_without_bom() {
    let fixture = Fixture::new(FILE_MATCHES);
    fixture.write_bytes(
        "abgabe/systeminfo.txt",
        &utf16le("Total Physical Memory: 8.192 MB\r\n", false),
    );
    assert!(fixture.run_first().passed);
}

#[test]
fn file_matches_cp850() {
    let fixture = Fixture::new(FILE_MATCHES);
    // German cmd.exe output: "Gesamter physischer Speicher" plus an umlaut line
    let mut bytes = b"Gesamter physischer Speicher: 16.384 MB\r\nGr".to_vec();
    bytes.extend_from_slice(&[0x94, 0xE1]); // ö, ß in CP850
    bytes.extend_from_slice(b"e: 500 GB\r\n");
    let fixture = {
        fixture.write_bytes("abgabe/systeminfo.txt", &bytes);
        fixture
    };
    assert!(fixture.run_first().passed);
}

#[test]
fn file_matches_reports_missing_pattern() {
    let fixture = Fixture::new(FILE_MATCHES);
    fixture.write(
        "abgabe/systeminfo.txt",
        "Nur eine Zeile ohne das Gesuchte\n",
    );
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::PatternNotFound {
            path: "abgabe/systeminfo.txt".into()
        })
    );
}

#[test]
fn file_matches_reports_missing_file() {
    let fixture = Fixture::new(FILE_MATCHES);
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert!(matches!(outcome.detail, Some(Detail::FileMissing { .. })));
}

// --- datei_zeilen_min -----------------------------------------------------

const MIN_LINES: &str = r#"
[[check]]
id = "genug-zeilen"
type = "datei_zeilen_min"
path = "abgabe/liste.txt"
min = 3
hint_de = "Schreibe mindestens drei Zeilen."
"#;

#[test]
fn min_lines_counts_only_non_empty_lines() {
    let fixture = Fixture::new(MIN_LINES);
    fixture.write("abgabe/liste.txt", "eins\n\n   \nzwei\n");
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::TooFewLines {
            path: "abgabe/liste.txt".into(),
            found: 2,
            min: 3
        })
    );
}

#[test]
fn min_lines_passes_with_enough_content() {
    let fixture = Fixture::new(MIN_LINES);
    fixture.write("abgabe/liste.txt", "eins\r\nzwei\r\ndrei\r\n");
    assert!(fixture.run_first().passed);
}

// --- antwort --------------------------------------------------------------

fn answer_fixture(salt: &str, hashes: &[String]) -> Fixture {
    let list = hashes
        .iter()
        .map(|h| format!("\"{h}\""))
        .collect::<Vec<_>>()
        .join(", ");
    Fixture::new(&format!(
        r#"
[[check]]
id = "frage-farbe"
type = "antwort"
key = "farbe"
salt = "{salt}"
expect_hash = [{list}]
hint_de = "Schau noch einmal in deine Notizen."
"#
    ))
}

#[test]
fn answer_matches_after_normalisation() {
    let salt = "test:01";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write("abgabe/antworten.toml", "farbe = \"  HIMMELBLAU  \"\n");
    assert!(
        fixture.run_first().passed,
        "trim + lowercase must be applied before hashing"
    );
}

#[test]
fn answer_accepts_several_spellings() {
    let salt = "test:02";
    let fixture = answer_fixture(
        salt,
        &[
            answers::hash("himmelblau", salt),
            answers::hash("hell blau", salt),
        ],
    );
    fixture.write("abgabe/antworten.toml", "farbe = \"hell   blau\"\n");
    assert!(
        fixture.run_first().passed,
        "whitespace inside the answer must be collapsed"
    );
}

#[test]
fn answer_wrong_value_reports_key_only() {
    let salt = "test:03";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write("abgabe/antworten.toml", "farbe = \"grün\"\n");
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::AnswerWrong {
            key: "farbe".into()
        })
    );
}

#[test]
fn answer_missing_key() {
    let salt = "test:04";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write("abgabe/antworten.toml", "andere = \"x\"\n");
    let outcome = fixture.run_first();
    assert!(matches!(outcome.detail, Some(Detail::AnswerMissing { .. })));
}

#[test]
fn answer_empty_value_counts_as_missing() {
    let salt = "test:05";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write("abgabe/antworten.toml", "farbe = \"   \"\n");
    assert!(matches!(
        fixture.run_first().detail,
        Some(Detail::AnswerMissing { .. })
    ));
}

#[test]
fn answer_missing_file() {
    let salt = "test:06";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    assert!(matches!(
        fixture.run_first().detail,
        Some(Detail::AnswersMissing { .. })
    ));
}

#[test]
fn answer_broken_file_is_reported_kindly() {
    let salt = "test:07";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write("abgabe/antworten.toml", "farbe = himmelblau\n");
    assert!(matches!(
        fixture.run_first().detail,
        Some(Detail::AnswersBroken { .. })
    ));
}

#[test]
fn answer_file_in_utf16le_is_read() {
    let salt = "test:08";
    let fixture = answer_fixture(salt, &[answers::hash("himmelblau", salt)]);
    fixture.write_bytes(
        "abgabe/antworten.toml",
        &utf16le("farbe = \"himmelblau\"\r\n", true),
    );
    assert!(
        fixture.run_first().passed,
        "Notepad/PowerShell may save answers as UTF-16LE"
    );
}

// --- alle_antworten -------------------------------------------------------

const ALL_ANSWERS: &str = r#"
[[check]]
id = "reflexion-vollstaendig"
type = "alle_antworten"
keys = ["r1", "r2", "r3"]
hint_de = "Beantworte alle Reflexionsfragen."
"#;

#[test]
fn all_answers_lists_what_is_missing() {
    let fixture = Fixture::new(ALL_ANSWERS);
    fixture.write("abgabe/antworten.toml", "r1 = \"ja\"\nr2 = \"   \"\n");
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::AnswersKeysMissing {
            path: "abgabe/antworten.toml".into(),
            keys: vec!["r2".into(), "r3".into()]
        })
    );
}

#[test]
fn all_answers_passes_when_complete() {
    let fixture = Fixture::new(ALL_ANSWERS);
    fixture.write(
        "abgabe/antworten.toml",
        "r1 = \"ja\"\nr2 = \"nein\"\nr3 = \"vielleicht\"\n",
    );
    assert!(fixture.run_first().passed);
}

// --- werte_gleich ---------------------------------------------------------

const VALUES_EQUAL: &str = r#"
[[check]]
id = "hash-beweis"
type = "werte_gleich"
key_a = "hash_vorher"
key_b = "hash_nachher"
hint_de = "Vergleiche die beiden Prüfsummen noch einmal."
"#;

#[test]
fn values_equal_ignores_case_and_spaces() {
    let fixture = Fixture::new(VALUES_EQUAL);
    fixture.write(
        "abgabe/antworten.toml",
        "hash_vorher = \"AB12CD\"\nhash_nachher = \"  ab12cd \"\n",
    );
    assert!(fixture.run_first().passed);
}

#[test]
fn values_equal_detects_difference() {
    let fixture = Fixture::new(VALUES_EQUAL);
    fixture.write(
        "abgabe/antworten.toml",
        "hash_vorher = \"ab12cd\"\nhash_nachher = \"ff99ee\"\n",
    );
    let outcome = fixture.run_first();
    assert!(!outcome.passed);
    assert_eq!(
        outcome.detail,
        Some(Detail::ValuesDiffer {
            key_a: "hash_vorher".into(),
            key_b: "hash_nachher".into()
        })
    );
}

#[test]
fn values_equal_reports_the_missing_key() {
    let fixture = Fixture::new(VALUES_EQUAL);
    fixture.write("abgabe/antworten.toml", "hash_vorher = \"ab12cd\"\n");
    assert_eq!(
        fixture.run_first().detail,
        Some(Detail::AnswerMissing {
            path: "abgabe/antworten.toml".into(),
            key: "hash_nachher".into()
        })
    );
}

// --- path escapes ---------------------------------------------------------

#[test]
fn path_escapes_are_rejected_at_load_time() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("01-boese-uebung");
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join("exercise.toml"),
        r#"
[exercise]
id = "01-boese-uebung"
titel = "Böse Übung"
modul = "test"
schwierigkeit = 1
zeit_minuten = 5
ki_stufe = "ohne"

[[check]]
id = "escape"
type = "file_exists"
path = "../../../etc/passwd"
hint_de = "nope"
"#,
    )
    .unwrap();
    let errors = exercise::load(&path).unwrap_err();
    assert!(
        errors.iter().any(|e| e.contains("..")),
        "expected a path-escape error, got {errors:?}"
    );
}

#[cfg(unix)]
#[test]
fn symlinks_out_of_the_exercise_are_rejected_at_run_time() {
    let fixture = Fixture::new(FILE_EXISTS);
    let outside = fixture.dir().parent().unwrap().join("geheim.txt");
    std::fs::write(&outside, "geheimer Inhalt").unwrap();
    std::os::unix::fs::symlink(&outside, fixture.dir().join("abgabe/notiz.txt")).unwrap();
    let outcome = fixture.run_first();
    assert!(
        !outcome.passed,
        "a symlink out of the exercise must not pass"
    );
    assert_eq!(
        outcome.detail,
        Some(Detail::PathEscape {
            path: "abgabe/notiz.txt".into()
        })
    );
}

#[cfg(windows)]
#[test]
fn junctions_out_of_the_exercise_are_rejected_at_run_time() {
    let fixture = Fixture::new(
        r#"
[[check]]
id = "datei-da"
type = "file_exists"
path = "abgabe/extern/notiz.txt"
hint_de = "Lege die Datei abgabe/extern/notiz.txt an."
"#,
    );
    let outside = fixture.dir().parent().unwrap().join("aussen");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("notiz.txt"), "geheimer Inhalt").unwrap();
    // A junction, unlike an NTFS symlink, needs neither admin rights nor
    // developer mode — it is the escape a learner VM can actually produce.
    let status = std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(fixture.dir().join("abgabe/extern"))
        .arg(&outside)
        .status()
        .expect("cmd /C mklink must be runnable on Windows");
    assert!(status.success(), "creating the junction failed");
    let outcome = fixture.run_first();
    assert!(
        !outcome.passed,
        "a junction out of the exercise must not pass"
    );
    assert_eq!(
        outcome.detail,
        Some(Detail::PathEscape {
            path: "abgabe/extern/notiz.txt".into()
        })
    );
}

// --- levels ---------------------------------------------------------------

#[test]
fn only_basis_checks_decide_whether_an_exercise_is_passed() {
    let fixture = Fixture::new(
        r#"
[[check]]
id = "basis-datei"
type = "file_exists"
path = "abgabe/notiz.txt"
hint_de = "Lege abgabe/notiz.txt an."

[[check]]
id = "bonus-datei"
type = "file_exists"
stufe = "bonus"
path = "abgabe/extra.txt"
hint_de = "Freiwillig: abgabe/extra.txt."

[[check]]
id = "homelab-datei"
type = "file_exists"
stufe = "homelab"
path = "abgabe/homelab.txt"
hint_de = "Nur mit eigenem Homelab."
"#,
    );
    fixture.write("abgabe/notiz.txt", "Hallo");

    let (exercise, result) = fixture.result();
    assert!(result.is_passed(&exercise), "basis complete = passed");
    assert_eq!(
        result.tally(&exercise, Level::Basis),
        Tally {
            passed: 1,
            total: 1
        }
    );
    assert_eq!(
        result.tally(&exercise, Level::Bonus),
        Tally {
            passed: 0,
            total: 1
        }
    );
    assert_eq!(
        result.tally(&exercise, Level::Homelab),
        Tally {
            passed: 0,
            total: 1
        }
    );
}
