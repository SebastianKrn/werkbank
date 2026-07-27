//! Integration tests: the real `wb` binary against fixture exercises.
//!
//! The centre of gravity is `learner_happy_path` — the exact sequence a person
//! in the classroom walks through. If that test is green, the mechanics work.

use std::path::{Path, PathBuf};
use std::process::Output;

use assert_cmd::Command;
use tempfile::TempDir;

/// A throwaway copy of the demo module, so tests never touch the repository.
struct Sandbox {
    dir: TempDir,
}

impl Sandbox {
    fn new() -> Self {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("modul-demo");
        let dir = tempfile::tempdir().expect("tempdir");
        copy_dir(&source, dir.path());
        Self { dir }
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn exercise(&self, id: &str) -> PathBuf {
        self.dir.path().join("uebungen").join(id)
    }

    fn write(&self, relative: &str, content: &str) {
        let target = self.dir.path().join(relative);
        std::fs::create_dir_all(target.parent().expect("parent")).expect("create dirs");
        std::fs::write(target, content).expect("write");
    }

    fn wb(&self) -> Command {
        let mut command = Command::cargo_bin("wb").expect("binary wb");
        command.current_dir(self.dir.path());
        command
    }
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("create target dir");
    for entry in std::fs::read_dir(from).expect("read source dir").flatten() {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).expect("copy file");
        }
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

// ---------------------------------------------------------------------------
// The path a learner actually walks
// ---------------------------------------------------------------------------

#[test]
fn learner_happy_path() {
    let sandbox = Sandbox::new();

    // 1. "Where am I?"
    let output = sandbox.wb().arg("status").output().expect("run status");
    assert_eq!(output.status.code(), Some(0));
    let text = stdout(&output);
    assert!(text.contains("Werkbank — Modul demo"), "{text}");
    assert!(text.contains("01-erste-schritte"), "{text}");
    assert!(
        text.contains("0 von 3 Übungen bestanden"),
        "nothing is done yet:\n{text}"
    );
    assert!(
        text.contains("wb check 01-erste-schritte"),
        "status must name the next step:\n{text}"
    );

    // 2. First check without any work: fails, but with a hint and no shaming.
    let output = sandbox.wb().arg("check").output().expect("run check");
    assert_eq!(output.status.code(), Some(1), "open exercise exits 1");
    let text = stdout(&output);
    assert!(text.contains("Übung 01-erste-schritte"), "{text}");
    assert!(
        text.contains("Hinweis:"),
        "a failing check shows a hint:\n{text}"
    );
    assert!(
        text.contains("Die Datei \"abgabe/notiz.txt\" gibt es noch nicht."),
        "{text}"
    );
    assert!(text.contains("Noch nicht fertig"), "{text}");
    for forbidden in ["Fehler!", "falsch", "Error", "failed"] {
        assert!(
            !text.contains(forbidden),
            "feedback must not shame or leak English: `{forbidden}`\n{text}"
        );
    }

    // 3. The learner does the work.
    sandbox.write(
        "uebungen/01-erste-schritte/abgabe/notiz.txt",
        "Ich habe eine Datei angelegt.\nDas Speichern hat geklappt.\nOffen ist noch Übung 2.\n",
    );

    // 4. Check again: passed, and pointed at the next exercise.
    let output = sandbox.wb().arg("check").output().expect("run check");
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    let text = stdout(&output);
    assert!(text.contains("Sehr gut!"), "{text}");
    assert!(
        text.contains("wb check 02-antworten-ueben"),
        "must point at the next exercise:\n{text}"
    );

    // 5. Exercise 2: answers file.
    sandbox.write(
        "uebungen/02-antworten-ueben/abgabe/antworten.toml",
        "werkzeug = \"Notepad\"\ngelernt = \"Dateien anlegen\"\n\
         pruefsumme_vorher = \"abc123\"\npruefsumme_nachher = \"ABC123\"\n",
    );
    let output = sandbox
        .wb()
        .args(["check", "02"])
        .output()
        .expect("run check 02");
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    let text = stdout(&output);
    assert!(
        text.contains("Bonus"),
        "the open bonus check must stay visible:\n{text}"
    );

    // 6. Exercise 3 via `wb erfasse`.
    let output = sandbox
        .wb()
        .args(["erfasse", "ordnerliste", "03"])
        .output()
        .expect("run erfasse");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(sandbox
        .exercise("03-ausgabe-erfassen")
        .join("abgabe/ordnerliste.txt")
        .is_file());

    let output = sandbox
        .wb()
        .args(["check", "03"])
        .output()
        .expect("run check 03");
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout(&output).contains("Du hast alle Übungen geschafft."));

    // 7. Hand-in report.
    let output = sandbox
        .wb()
        .args(["bericht", "--alias", "Testperson"])
        .output()
        .expect("run bericht");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));

    let report = std::fs::read_to_string(sandbox.path().join("bericht.txt")).expect("bericht.txt");
    assert!(report.contains("Testperson"), "{report}");
    assert!(report.contains("3 von 3 Übungen bestanden"), "{report}");
    assert!(report.contains("Prüfsumme:"), "{report}");
    assert!(
        report.contains("kein Schutz gegen absichtliche Manipulation"),
        "the report must state its own limits:\n{report}"
    );

    let json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(sandbox.path().join("bericht.json")).expect("bericht.json"),
    )
    .expect("valid json");
    assert_eq!(json["alias"], "Testperson");
    assert_eq!(json["bestanden"], 3);
    assert_eq!(json["lb_bestanden"], 2);
    assert_eq!(json["pruefsumme"].as_str().unwrap().len(), 64);

    // 8. Progress stayed local.
    assert!(sandbox.path().join(".werkbank/fortschritt.json").is_file());
}

// ---------------------------------------------------------------------------
// Level model
// ---------------------------------------------------------------------------

#[test]
fn bonus_and_homelab_never_block_an_exercise() {
    let sandbox = Sandbox::new();
    sandbox.write(
        "uebungen/01-erste-schritte/abgabe/notiz.txt",
        "eins\nzwei\ndrei\n",
    );
    let output = sandbox
        .wb()
        .args(["check", "01", "--json"])
        .output()
        .expect("run check");
    assert_eq!(output.status.code(), Some(0));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("valid json");
    assert_eq!(json["bestanden"], true);
    assert_eq!(json["stufen"]["basis"]["erfuellt"], 2);
    assert_eq!(
        json["stufen"]["bonus"]["erfuellt"], 0,
        "bonus is open and that is fine"
    );
    let bonus = json["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["stufe"] == "bonus")
        .expect("bonus check in json");
    assert!(bonus["hinweis"].is_string(), "open checks carry their hint");
}

#[test]
fn bare_check_confirms_the_work_just_done_before_moving_on() {
    let sandbox = Sandbox::new();
    sandbox.wb().arg("check").output().expect("first check");
    sandbox.write(
        "uebungen/01-erste-schritte/abgabe/notiz.txt",
        "eins\nzwei\ndrei\n",
    );

    // The run right after fixing must report exercise 01, not jump ahead.
    let text = stdout(&sandbox.wb().arg("check").output().expect("second check"));
    assert!(text.contains("Übung 01-erste-schritte"), "{text}");
    assert!(text.contains("Sehr gut!"), "{text}");

    // Only the run after that moves on.
    let text = stdout(&sandbox.wb().arg("check").output().expect("third check"));
    assert!(text.contains("Übung 02-antworten-ueben"), "{text}");
}

#[test]
fn status_json_reports_every_exercise() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["status", "--json"])
        .output()
        .expect("run status");
    assert_eq!(output.status.code(), Some(0));

    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("valid json");
    assert_eq!(json["gesamt"], 3);
    assert_eq!(json["bestanden"], 0);
    assert_eq!(json["naechste"], "01-erste-schritte");
    assert_eq!(json["kaputt"].as_array().unwrap().len(), 0);
}

// ---------------------------------------------------------------------------
// Being kind when something is wrong
// ---------------------------------------------------------------------------

#[test]
fn unknown_exercise_lists_what_exists() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["check", "42-gibt-es-nicht"])
        .output()
        .expect("run check");
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert!(text.contains("finde ich nicht"), "{text}");
    assert!(text.contains("01-erste-schritte"), "{text}");
}

#[test]
fn missing_uebungen_folder_explains_the_zip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("wb").expect("binary wb");
    let output = command
        .current_dir(dir.path())
        .arg("status")
        .output()
        .expect("run status");
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert!(text.contains("uebungen"), "{text}");
    assert!(text.contains("entpack"), "{text}");
}

#[test]
fn unknown_command_answers_in_german() {
    let sandbox = Sandbox::new();
    let output = sandbox.wb().arg("gibtsnicht").output().expect("run");
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert!(text.contains("Diesen Befehl kenne ich nicht."), "{text}");
    assert!(
        text.contains("wb status"),
        "help follows the error:\n{text}"
    );
}

#[test]
fn wb_works_from_inside_an_exercise_folder() {
    let sandbox = Sandbox::new();
    let mut command = Command::cargo_bin("wb").expect("binary wb");
    let output = command
        .current_dir(sandbox.exercise("02-antworten-ueben"))
        .arg("status")
        .output()
        .expect("run status");
    assert_eq!(
        output.status.code(),
        Some(0),
        "wb must find the root by walking up:\n{}",
        stderr(&output)
    );
    assert!(stdout(&output).contains("Modul demo"));
}

// ---------------------------------------------------------------------------
// Help, refusal, dedication
// ---------------------------------------------------------------------------

#[test]
fn bare_wb_and_hilfe_show_the_same_german_help() {
    let sandbox = Sandbox::new();
    let bare = stdout(&sandbox.wb().output().expect("run wb"));
    let hilfe = stdout(&sandbox.wb().arg("hilfe").output().expect("run wb hilfe"));
    assert_eq!(bare, hilfe);
    for expected in [
        "wb status",
        "wb check",
        "wb bericht",
        "Fehlversuch kostet nichts",
    ] {
        assert!(
            bare.contains(expected),
            "missing `{expected}` in help:\n{bare}"
        );
    }
    assert!(
        !bare.contains("deo-gratias"),
        "the easter egg stays out of the help"
    );
}

#[test]
fn every_help_screen_is_completely_german() {
    let sandbox = Sandbox::new();
    for args in [
        vec!["--help"],
        vec!["-h"],
        vec!["check", "--help"],
        vec!["erfasse", "--help"],
        vec!["bericht", "--help"],
        vec!["status", "--help"],
        vec!["loesung", "--help"],
    ] {
        let output = sandbox.wb().args(&args).output().expect("run help");
        assert_eq!(output.status.code(), Some(0), "{args:?}");
        let text = stdout(&output);
        for english in [
            "Usage:",
            "Options:",
            "Commands:",
            "Arguments:",
            "Print help",
            "Print version",
        ] {
            assert!(
                !text.contains(english),
                "`wb {}` leaks English (`{english}`):\n{text}",
                args.join(" ")
            );
        }
        assert!(text.contains("Verwendung:"), "{text}");
    }
}

#[test]
fn loesung_refuses_and_points_somewhere_useful() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["loesung", "01"])
        .output()
        .expect("run loesung");
    assert_eq!(output.status.code(), Some(0));
    let text = stdout(&output);
    assert!(text.contains("keine Lösung"), "{text}");
    assert!(text.contains("wb check 01-erste-schritte"), "{text}");
    assert!(text.contains("Trainer"), "{text}");
}

#[test]
fn deo_gratias_exists_but_is_not_advertised() {
    let sandbox = Sandbox::new();
    let output = sandbox.wb().arg("deo-gratias").output().expect("run");
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout(&output).contains("Soli Deo gloria"));
}

// ---------------------------------------------------------------------------
// erfasse
// ---------------------------------------------------------------------------

#[test]
fn erfasse_without_a_name_lists_the_presets() {
    let sandbox = Sandbox::new();
    let output = sandbox.wb().arg("erfasse").output().expect("run erfasse");
    assert_eq!(output.status.code(), Some(0));
    let text = stdout(&output);
    for preset in [
        "systeminfo",
        "ipconfig",
        "hardware",
        "firmware",
        "datentraeger",
        "spiegel",
        "bitlocker",
        "schutz",
        "ordnerliste",
    ] {
        assert!(text.contains(preset), "preset `{preset}` missing:\n{text}");
    }
}

#[test]
fn erfasse_runs_a_real_compiled_in_command() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["erfasse", "ipconfig", "01"])
        .output()
        .expect("run erfasse");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let written = sandbox
        .exercise("01-erste-schritte")
        .join("abgabe/ipconfig.txt");
    assert!(written.is_file(), "capture file was not written");
    assert!(
        !std::fs::read_to_string(&written)
            .expect("read capture")
            .is_empty(),
        "capture file is empty"
    );
}

#[test]
fn erfasse_rejects_an_unknown_preset() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["erfasse", "geheimnisse"])
        .output()
        .expect("run erfasse");
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("kenne ich nicht"));
}

#[test]
fn erfasse_ordnerliste_refuses_to_leave_the_exercise() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["erfasse", "ordnerliste", "01", "--ordner", "../.."])
        .output()
        .expect("run erfasse");
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("außerhalb"), "{}", stderr(&output));
}

// ---------------------------------------------------------------------------
// bericht
// ---------------------------------------------------------------------------

#[test]
fn bericht_asks_for_a_name_once_and_remembers_it() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .arg("bericht")
        .write_stdin("Rafaela\n")
        .output()
        .expect("run bericht");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stdout(&output).contains("Wie sollen wir dich im Bericht nennen?"));

    // Second run must not ask again.
    let output = sandbox.wb().arg("bericht").output().expect("run bericht");
    assert_eq!(output.status.code(), Some(0));
    assert!(!stdout(&output).contains("Wie sollen wir dich"));
    let report = std::fs::read_to_string(sandbox.path().join("bericht.txt")).expect("bericht.txt");
    assert!(report.contains("Rafaela"), "{report}");
}

// ---------------------------------------------------------------------------
// wb intern lint (content is code)
// ---------------------------------------------------------------------------

#[test]
fn lint_accepts_the_demo_module() {
    let mut command = Command::cargo_bin("wb").expect("binary wb");
    let output = command
        .args(["intern", "lint"])
        .arg(fixture_root().join("modul-demo").join("uebungen"))
        .output()
        .expect("run lint");
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    let text = stdout(&output);
    assert!(text.contains("3 exercise(s) checked, all valid."), "{text}");
}

#[test]
fn lint_rejects_every_invalid_fixture() {
    let cases = [
        ("pfad-escape", "`..` segments"),
        ("kaputte-regex", "does not compile"),
        ("unbekanntes-feld", "unknown field `punkte`"),
        ("feld-passt-nicht", "has no meaning for type"),
        ("falsche-ki-stufe", "ki_stufe `manchmal`"),
        ("ohne-basis-check", "no check with stufe `basis`"),
        ("id-passt-nicht", "must match the folder name"),
        ("windows-geraetename", "reserved Windows device name"),
    ];
    for (case, expected) in cases {
        let mut command = Command::cargo_bin("wb").expect("binary wb");
        let output = command
            .args(["intern", "lint"])
            .arg(fixture_root().join("ungueltig").join(case).join("uebungen"))
            .output()
            .expect("run lint");
        assert_eq!(
            output.status.code(),
            Some(2),
            "`{case}` must fail lint:\n{}",
            stdout(&output)
        );
        assert!(
            stdout(&output).contains(expected),
            "`{case}` should mention `{expected}`:\n{}",
            stdout(&output)
        );
    }
}

#[test]
fn lint_fails_when_there_is_nothing_to_lint() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut command = Command::cargo_bin("wb").expect("binary wb");
    let output = command
        .args(["intern", "lint"])
        .arg(dir.path())
        .output()
        .expect("run lint");
    assert_eq!(
        output.status.code(),
        Some(2),
        "an empty folder must not pass silently in CI"
    );
}

#[test]
fn intern_hash_prints_only_hashes() {
    let mut command = Command::cargo_bin("wb").expect("binary wb");
    let output = command
        .args([
            "intern",
            "hash",
            "--salt",
            "wb1:01",
            "SSD",
            "solid state disk",
        ])
        .output()
        .expect("run hash");
    assert_eq!(output.status.code(), Some(0));
    let text = stdout(&output);
    assert!(text.contains("expect_hash = ["), "{text}");
    assert!(
        !text.contains("SSD") && !text.to_lowercase().contains("solid state"),
        "plaintext answers must never reach stdout:\n{text}"
    );
    assert_eq!(
        text.matches("\",").count(),
        2,
        "one hash per answer:\n{text}"
    );
}

// ---------------------------------------------------------------------------
// Broken content must not take the module down
// ---------------------------------------------------------------------------

#[test]
fn a_broken_exercise_does_not_hide_the_others() {
    let sandbox = Sandbox::new();
    sandbox.write(
        "uebungen/04-kaputt/exercise.toml",
        "[exercise]\nid = \"04-kaputt\"\n",
    );

    let output = sandbox.wb().arg("status").output().expect("run status");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("01-erste-schritte"), "{text}");
    assert!(text.contains("kann ich nicht lesen"), "{text}");

    // Asking for it by name explains the real problem instead of "unknown".
    let output = sandbox
        .wb()
        .args(["check", "04-kaputt"])
        .output()
        .expect("run check");
    assert_eq!(output.status.code(), Some(2));
    assert!(
        stderr(&output).contains("kann ich nicht lesen"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn ascii_mode_avoids_symbols() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .wb()
        .args(["status", "--ascii"])
        .output()
        .expect("run status");
    assert_eq!(output.status.code(), Some(0));
    let text = stdout(&output);
    assert!(text.contains("[  ]"), "{text}");
    assert!(!text.contains('⬜'), "{text}");
}
