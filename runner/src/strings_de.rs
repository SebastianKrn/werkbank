//! Every sentence a learner or trainer ever sees.
//!
//! Rules (CLAUDE.md rule 4 / PRD §7.3): German, Austrian, simple language
//! (~B1), short sentences, encouraging, never shaming. Hints, never solutions.
//! No information carried by colour or symbol alone — every marker is paired
//! with a German word.
//!
//! Keeping the strings here (and not inline) is what makes the module swap for
//! a future second language a content change instead of a refactor.

use crate::checks::{Detail, Tally};
use crate::exercise::{AiLevel, Level};

// ---------------------------------------------------------------------------
// Markers
// ---------------------------------------------------------------------------

/// Status markers. Unicode by default; `--ascii` switches to a plain set for
/// consoles with a narrow font or an old code page (checked again in M3 on a
/// real VM).
#[derive(Debug, Clone, Copy)]
pub struct Symbols {
    pub done: &'static str,
    pub started: &'static str,
    pub open: &'static str,
    width: usize,
}

pub const UNICODE: Symbols = Symbols {
    done: "✅",
    started: "🔨",
    open: "⬜",
    width: 2,
};

pub const ASCII: Symbols = Symbols {
    done: "[ok]",
    started: "[..]",
    open: "[  ]",
    width: 4,
};

impl Symbols {
    pub fn set(ascii: bool) -> Self {
        if ascii {
            ASCII
        } else {
            UNICODE
        }
    }

    /// Marker padded to a fixed column width so rows line up.
    pub fn cell(&self, marker: &str) -> String {
        let printed = if marker == self.done || marker == self.started || marker == self.open {
            self.width
        } else {
            marker.chars().count()
        };
        let pad = ASCII.width.saturating_sub(printed) + 1;
        format!("{marker}{}", " ".repeat(pad))
    }
}

// ---------------------------------------------------------------------------
// Help
// ---------------------------------------------------------------------------

pub const KURZBESCHREIBUNG: &str = "Werkbank — Übungen prüfen und Fortschritt zeigen";

/// How the learner has to type the binary — as a literal, so that `concat!`
/// can build clap's usage lines at compile time.
///
/// PowerShell, the shell `START_HIER.md` tells them to open, does not search
/// the current directory. A bare `wb check 01` answers „Die Benennung 'wb'
/// wurde nicht als Name eines Cmdlets … erkannt", and the troubleshooting
/// table in `START_HIER.md` then sends them hunting for the wrong folder.
/// Every command this runner suggests must be typeable exactly as printed.
#[cfg(windows)]
macro_rules! wb_befehl {
    () => {
        ".\\wb"
    };
}
#[cfg(not(windows))]
macro_rules! wb_befehl {
    () => {
        "./wb"
    };
}

pub const WB: &str = wb_befehl!();

/// Layout for clap's `--help`. clap's own headings are English, so they are
/// replaced here — a learner must never meet an English word (rule 4).
pub const HILFE_VORLAGE: &str = concat!(
    "\
{about-with-newline}
Verwendung: {usage}

{all-args}

Ausführliche Hilfe:  ",
    wb_befehl!(),
    " hilfe"
);

pub const HILFE_VORLAGE_BEFEHL: &str = "\
{about-with-newline}
Verwendung: {usage}

{all-args}";

pub const UEBERSCHRIFT_BEFEHLE: &str = "Befehle";
pub const UEBERSCHRIFT_OPTIONEN: &str = "Optionen";
pub const UEBERSCHRIFT_ANGABEN: &str = "Angaben";

pub const NUTZUNG_WB: &str = concat!(wb_befehl!(), " [BEFEHL] [OPTIONEN]");
pub const NUTZUNG_STATUS: &str = concat!(wb_befehl!(), " status [--json] [--ascii]");
pub const NUTZUNG_CHECK: &str = concat!(wb_befehl!(), " check [ID] [--json] [--ascii]");
pub const NUTZUNG_ERFASSE: &str = concat!(wb_befehl!(), " erfasse [NAME] [ID] [--ordner PFAD]");
pub const NUTZUNG_BERICHT: &str = concat!(wb_befehl!(), " bericht [--alias NAME]");
pub const NUTZUNG_LOESUNG: &str = concat!(wb_befehl!(), " loesung <ID>");
pub const NUTZUNG_HILFE: &str = concat!(wb_befehl!(), " hilfe");

pub const ARG_UEBUNG_ID: &str = "Nummer oder Name der Übung, z. B. 01";
pub const ARG_ERFASSE_NAME: &str = "Name der Ausgabe, z. B. systeminfo";

pub const CMD_STATUS: &str = "Zeigt alle Übungen und wo du stehst";
pub const CMD_CHECK: &str = "Prüft eine Übung und gibt dir Hinweise";
pub const CMD_ERFASSE: &str = "Speichert eine Systemausgabe in deine Abgabe";
pub const CMD_BERICHT: &str = "Schreibt deinen Bericht für den Unterricht";
pub const CMD_LOESUNG: &str = "Erklärt, warum es hier keine Lösungen zum Nachschauen gibt";
pub const CMD_HILFE: &str = "Zeigt die Hilfe";

pub const FLAG_HILFE: &str = "Zeigt die Hilfe zu diesem Befehl";
pub const FLAG_VERSION: &str = "Zeigt die Version von wb";
pub const FLAG_JSON: &str = "Ausgabe für Maschinen statt für Menschen";
pub const FLAG_ASCII: &str = "Einfache Zeichen statt Symbolen";
pub const FLAG_ORDNER: &str = "Ordner in der Übung (nur bei ordnerliste)";
pub const FLAG_ALIAS: &str = "Dein Name oder Kürzel für den Bericht";

pub fn hilfe() -> String {
    /// Width of the command column, so the descriptions line up whatever the
    /// platform prefix costs.
    const SPALTE: usize = 22;
    let befehl = |rest: &str| format!("  {:<SPALTE$}", format!("{WB} {rest}"));

    format!(
        "\
Werkbank — deine Übungswerkstatt

So arbeitest du:
  1. Übung lesen        Öffne AUFGABE.md im Ordner der Übung.
  2. Aufgabe machen     Lege deine Dateien in den Ordner \"abgabe\".
  3. Prüfen lassen      Tippe: {WB} check

Befehle:
{}Zeigt alle Übungen und wo du stehst.
{}Prüft die Übung, an der du gerade bist.
{}Prüft eine bestimmte Übung, z. B.: {WB} check 01
{}Speichert eine Systemausgabe in deine Abgabe,
  {:<SPALTE$}z. B.: {WB} erfasse systeminfo
{}Zeigt, welche Ausgaben es gibt.
{}Schreibt deinen Bericht für den Unterricht (bericht.txt).
{}Erklärt, warum es hier keine Lösungen zum Nachschauen gibt.
{}Zeigt diese Hilfe.

Zusätzlich:
  --json                Ausgabe für Maschinen (bei status und check).
  --ascii               Einfache Zeichen statt Symbolen, falls dein Fenster
                        die Symbole nicht sauber anzeigt.

Gut zu wissen:
  Du kannst so oft prüfen, wie du willst. Ein Fehlversuch kostet nichts.
  Du bekommst immer einen Hinweis — nie die fertige Lösung.
",
        befehl("status"),
        befehl("check"),
        befehl("check <ID>"),
        befehl("erfasse <name>"),
        "",
        befehl("erfasse"),
        befehl("bericht"),
        befehl("loesung <ID>"),
        befehl("hilfe"),
    )
}

pub fn unbekannter_befehl() -> String {
    format!("Diesen Befehl kenne ich nicht.\n\nTippe \"{WB} hilfe\", dann siehst du alle Befehle.")
}

/// The command exists, something after it does not. Saying „kenne ich nicht"
/// here sends the learner looking for the wrong mistake — and the command they
/// just typed is printed in the help two lines further down.
pub fn befehl_unvollstaendig() -> String {
    format!(
        "Zu diesem Befehl fehlt noch eine Angabe.\n\n\
         Beispiel:  {WB} loesung 01\n\
         Welche Angaben ein Befehl braucht, steht unten."
    )
}

// ---------------------------------------------------------------------------
// Errors (learner facing)
// ---------------------------------------------------------------------------

pub fn kein_uebungsordner(gesucht: &str) -> String {
    format!(
        "Ich finde den Ordner \"{gesucht}\" nicht.\n\n\
         Bitte entpacke das ZIP und starte wb in dem entpackten Ordner.\n\
         Dort liegen die Datei wb (bzw. wb.exe) und der Ordner \"{gesucht}\" nebeneinander."
    )
}

pub fn keine_uebungen(gesucht: &str) -> String {
    format!("Im Ordner \"{gesucht}\" finde ich noch keine Übungen.")
}

pub fn uebung_unbekannt(id: &str, vorhanden: &[String]) -> String {
    let liste = vorhanden
        .iter()
        .map(|id| format!("  {id}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Eine Übung mit \"{id}\" finde ich nicht.\n\nDas gibt es:\n{liste}\n\n\
         Tipp: Die Nummer reicht, z. B.: {WB} check 01"
    )
}

pub fn uebung_mehrdeutig(id: &str, treffer: &[String]) -> String {
    let liste = treffer
        .iter()
        .map(|id| format!("  {id}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("\"{id}\" passt auf mehrere Übungen:\n{liste}\n\nBitte schreibe die Übung genauer.")
}

pub fn uebung_kaputt(pfad: &str, probleme: &[String]) -> String {
    let liste = probleme
        .iter()
        .map(|p| format!("  - {p}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Die Übungsdatei in \"{pfad}\" kann ich nicht lesen.\n\
         Das liegt nicht an dir. Bitte zeig deinem Trainer diese Meldung:\n\n{liste}"
    )
}

pub fn schreibfehler(pfad: &str, problem: &str) -> String {
    format!("Ich kann \"{pfad}\" nicht schreiben.\nGrund: {problem}")
}

/// `wb check` and `wb status` keep their result when only the bookkeeping
/// fails: the learner has done the work either way, and a beginner who is
/// shown a write error instead of „Sehr gut!" learns nothing from it.
pub fn fortschritt_nicht_gespeichert(problem: &str) -> String {
    format!(
        "Hinweis: Deinen Fortschritt konnte ich nicht speichern.\n\
         Das Ergebnis oben stimmt trotzdem — gemerkt wird es diesmal nur nicht.\n\
         Grund: {problem}\n\
         Sag deinem Trainer Bescheid, wenn das öfter vorkommt."
    )
}

// ---------------------------------------------------------------------------
// check
// ---------------------------------------------------------------------------

pub fn uebung_kopf(id: &str, titel: &str) -> String {
    format!("Übung {id} — {titel}")
}

pub fn uebung_meta(minuten: u32, schwierigkeit: u8, ki: AiLevel) -> String {
    format!(
        "ca. {minuten} Minuten · Schwierigkeit {schwierigkeit} von 3 · {}",
        ki_hinweis(ki)
    )
}

pub fn ki_hinweis(ki: AiLevel) -> &'static str {
    match ki {
        AiLevel::Ohne => "KI: diese Übung bitte ohne KI lösen",
        AiLevel::Danach => "KI: zuerst selbst, danach mit KI vergleichen",
        AiLevel::Frei => "KI: darfst du frei nutzen",
    }
}

pub const LB_HINWEIS: &str = "Diese Übung ist wichtig für die Leistungsbeurteilung.";

pub fn stufen_titel(level: Level) -> &'static str {
    match level {
        Level::Basis => "Basis (das brauchst du)",
        Level::Bonus => "Bonus (freiwillig, für mehr Tiefe)",
        Level::Homelab => "Homelab (freiwillig, für dein eigenes Labor)",
    }
}

pub fn stufen_kurz(level: Level) -> &'static str {
    match level {
        Level::Basis => "Basis",
        Level::Bonus => "Bonus",
        Level::Homelab => "Homelab",
    }
}

pub fn check_zeile(symbols: &Symbols, erfuellt: bool, id: &str) -> String {
    let marker = if erfuellt { symbols.done } else { symbols.open };
    let wort = if erfuellt { "erledigt" } else { "offen" };
    format!("  {}{id}  ({wort})", symbols.cell(marker))
}

pub fn hinweis_zeile(hinweis: &str) -> String {
    format!("       Hinweis: {hinweis}")
}

pub fn detail_zeile(detail: &Detail) -> String {
    format!("       {}", detail_text(detail))
}

/// German wording for everything a check can find.
pub fn detail_text(detail: &Detail) -> String {
    match detail {
        Detail::FileMissing { path } => {
            format!("Die Datei \"{path}\" gibt es noch nicht.")
        }
        Detail::FileEmpty { path } => {
            format!("Die Datei \"{path}\" ist leer.")
        }
        Detail::FileUnreadable { path, problem } => {
            format!("Die Datei \"{path}\" kann ich nicht lesen. (technisch: {problem})")
        }
        Detail::PatternNotFound { path } => {
            format!("In \"{path}\" fehlt noch etwas Wichtiges.")
        }
        Detail::TooFewLines { path, found, min } => {
            format!(
                "In \"{path}\" finde ich {}. Gebraucht werden {min}.",
                zeilen(*found)
            )
        }
        Detail::AnswersMissing { path } => {
            format!("Die Datei \"{path}\" gibt es noch nicht. Dort trägst du deine Antworten ein.")
        }
        Detail::AnswersBroken { path, problem } => {
            // The backslash trap is the one beginners really hit: a Windows path
            // inside double quotes ("C:\wb") is an invalid escape in TOML and
            // breaks the whole file, which would otherwise block every answer
            // check of the exercise with an unreadable message.
            let hinweis = if problem.contains("escape") {
                "\n       Steht in einer Antwort ein Pfad oder Befehl mit \\ ?\n       \
                 Dann nimm einfache Anführungszeichen:  schluessel = 'C:\\wb\\probe'"
            } else {
                ""
            };
            format!(
                "Die Datei \"{path}\" kann ich nicht lesen.\n       \
                 Achte auf diese Form:  schluessel = \"deine antwort\"{hinweis}\n       \
                 (technisch: {problem})"
            )
        }
        Detail::AnswersUnsupported { path, key } => {
            format!(
                "Der Eintrag \"{key}\" in \"{path}\" hat eine Form, die ich nicht vergleichen kann.\n       \
                 Schreibe eine einfache Antwort in Anführungszeichen."
            )
        }
        Detail::AnswerMissing { path, key } => {
            format!("In \"{path}\" fehlt noch der Eintrag \"{key}\".")
        }
        Detail::AnswerWrong { key } => {
            format!("Die Antwort bei \"{key}\" passt noch nicht.")
        }
        Detail::AnswersKeysMissing { path, keys } => {
            format!("In \"{path}\" fehlen noch: {}.", keys.join(", "))
        }
        Detail::ValuesDiffer { key_a, key_b } => {
            format!("\"{key_a}\" und \"{key_b}\" sind nicht gleich.")
        }
        Detail::PathEscape { path } => {
            format!(
                "Der Pfad \"{path}\" zeigt aus dem Übungsordner hinaus.\n       \
                 Aus Sicherheitsgründen prüfe ich das nicht. Bitte melde dich bei deinem Trainer."
            )
        }
    }
}

fn zeilen(anzahl: usize) -> String {
    if anzahl == 1 {
        "1 Zeile mit Text".to_string()
    } else {
        format!("{anzahl} Zeilen mit Text")
    }
}

pub fn tally_zeile(level: Level, tally: Tally) -> String {
    format!(
        "{}: {} von {} erledigt",
        stufen_kurz(level),
        tally.passed,
        tally.total
    )
}

pub fn check_geschafft(id: &str) -> String {
    format!("Sehr gut! Die Übung {id} ist geschafft.")
}

pub fn check_offen() -> String {
    "Noch nicht fertig — das ist völlig in Ordnung.\n\
     Nimm dir den ersten Hinweis von oben vor. Du schaffst das."
        .to_string()
}

pub fn naechster_schritt_check(id: &str) -> String {
    format!("Weiter geht es mit:  {WB} check {id}")
}

pub fn nochmal_pruefen(id: &str) -> String {
    format!("Wenn du so weit bist:  {WB} check {id}")
}

pub fn alles_geschafft() -> String {
    "Du hast alle Übungen geschafft. Respekt!\n\
     Erstelle jetzt deinen Bericht:  {WB} bericht"
        .to_string()
}

/// Optional further reading. Never required to pass (SPEC §3).
pub fn vertiefung(links: &[String]) -> String {
    let liste = links
        .iter()
        .map(|link| format!("  {link}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("Zum Weiterlesen (freiwillig, braucht Internet):\n{liste}")
}

pub fn bonus_offen() -> String {
    "Es gibt hier noch freiwillige Zusatz-Aufgaben. Du musst sie nicht machen.".to_string()
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

pub fn status_kopf(modul: &str) -> String {
    format!("Werkbank — Modul {modul}")
}

pub const STATUS_LB_MARKE: &str = "LB";

pub fn status_fortschritt(bestanden: usize, gesamt: usize) -> String {
    format!("Fortschritt: {bestanden} von {gesamt} Übungen bestanden")
}

pub fn fortschrittsbalken(bestanden: usize, gesamt: usize) -> String {
    let breite = 20usize;
    let voll = (bestanden * breite)
        .checked_div(gesamt)
        .unwrap_or(0)
        .min(breite);
    let prozent = (bestanden * 100).checked_div(gesamt).unwrap_or(0);
    format!(
        "[{}{}] {prozent} %",
        "#".repeat(voll),
        "-".repeat(breite.saturating_sub(voll))
    )
}

pub fn status_naechster(id: &str) -> String {
    format!("Dein nächster Schritt:  {WB} check {id}")
}

pub fn status_lb_zeile(bestanden: usize, gesamt: usize) -> String {
    format!("Davon prüfungsrelevant (LB): {bestanden} von {gesamt} bestanden")
}

pub fn status_kaputte_uebungen(pfade: &[String]) -> String {
    let liste = pfade
        .iter()
        .map(|p| format!("  - {p}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("Achtung: diese Übungen kann ich nicht lesen. Bitte deinem Trainer zeigen:\n{liste}")
}

// ---------------------------------------------------------------------------
// erfasse
// ---------------------------------------------------------------------------

pub fn erfasse_uebersicht(zeilen: &[(String, String, bool)]) -> String {
    let mut out = String::from(
        "{WB} erfasse — speichert eine Systemausgabe in deine Abgabe.\n\n\
         So geht es:  {WB} erfasse <name>\n\
         Beispiel:    {WB} erfasse systeminfo\n\n\
         Das gibt es:\n",
    );
    let breite = zeilen.iter().map(|(n, _, _)| n.len()).max().unwrap_or(0);
    for (name, beschreibung, verfuegbar) in zeilen {
        let marke = if *verfuegbar {
            String::new()
        } else {
            "  (nur unter Windows)".to_string()
        };
        out.push_str(&format!("  {name:breite$}  {beschreibung}{marke}\n"));
    }
    out.push_str(
        "\nDie Datei landet im Ordner \"abgabe\" der Übung.\n\
         Danach prüfen mit:  {WB} check\n",
    );
    out
}

pub fn erfasse_unbekannt(name: &str, bekannte: &[String]) -> String {
    format!(
        "Eine Ausgabe namens \"{name}\" kenne ich nicht.\n\nDas gibt es: {}\n\n\
         Tipp: \"{WB} erfasse\" ohne Namen zeigt die Liste mit Erklärung.",
        bekannte.join(", ")
    )
}

pub fn erfasse_nur_windows(name: &str) -> String {
    format!("\"{name}\" gibt es nur unter Windows. Auf diesem System kann ich das nicht erfassen.")
}

pub fn erfasse_laeuft(name: &str) -> String {
    format!("Ich erfasse \"{name}\". Das kann einen Moment dauern …")
}

pub fn erfasse_gespeichert(pfad: &str, zeilen: usize) -> String {
    format!("Gespeichert: {pfad} ({zeilen} Zeilen)")
}

pub fn erfasse_leer(pfad: &str) -> String {
    format!(
        "Der Befehl hat nichts ausgegeben. Ich habe \"{pfad}\" trotzdem angelegt.\n\
         Schau bitte nach, ob du das richtige System erwischt hast."
    )
}

pub fn erfasse_fehlgeschlagen(problem: &str) -> String {
    format!(
        "Der Befehl konnte nicht gestartet werden.\n\
         Technisch: {problem}\n\
         Du kannst die Ausgabe auch selbst speichern — schau in die AUFGABE.md."
    )
}

pub fn erfasse_meldete_fehler() -> String {
    "Achtung: der Befehl hat einen Fehler gemeldet. Ich habe die Ausgabe trotzdem gespeichert.\n\
     Schau kurz in die Datei, ob etwas Brauchbares drinsteht."
        .to_string()
}

pub fn erfasse_ordner_ausserhalb() -> String {
    "Dieser Ordner liegt außerhalb der Übung. Ich erfasse nur Ordner innerhalb des Übungsordners."
        .to_string()
}

pub fn erfasse_ordner_fehlt(pfad: &str) -> String {
    format!("Den Ordner \"{pfad}\" gibt es nicht.")
}

// ---------------------------------------------------------------------------
// bericht
// ---------------------------------------------------------------------------

pub const BERICHT_FRAGE_ALIAS: &str =
    "Wie sollen wir dich im Bericht nennen? (Vorname oder Kürzel reicht)";
pub const BERICHT_OHNE_NAME: &str = "ohne Namen";

pub fn bericht_geschrieben(txt: &str, json: &str) -> String {
    format!(
        "Bericht geschrieben:\n  {txt}\n  {json}\n\n\
         Gib die Datei bericht.txt bei deinem Trainer ab.\n\
         Sie enthält nur deinen Namen, deine Übungen und die Zeiten — sonst nichts."
    )
}

pub const BERICHT_TITEL: &str = "WERKBANK — BERICHT";
pub const BERICHT_FELD_NAME: &str = "Name/Kürzel";
pub const BERICHT_FELD_MODUL: &str = "Modul";
pub const BERICHT_FELD_ERSTELLT: &str = "Erstellt am";
pub const BERICHT_ABSCHNITT_UEBUNGEN: &str = "Übungen";
pub const BERICHT_ABSCHNITT_ZUSAMMENFASSUNG: &str = "Zusammenfassung";
pub const BERICHT_PRUEFSUMME_HINWEIS: &str =
    "Die Prüfsumme zeigt nur, ob die Datei nachträglich verändert wurde.\n\
     Sie ist kein Schutz gegen absichtliche Manipulation.";

pub fn bericht_status(bestanden: bool, begonnen: bool) -> &'static str {
    if bestanden {
        "bestanden"
    } else if begonnen {
        "begonnen"
    } else {
        "offen"
    }
}

pub fn versuche(anzahl: u32) -> String {
    if anzahl == 1 {
        "1 Versuch".to_string()
    } else {
        format!("{anzahl} Versuche")
    }
}

// ---------------------------------------------------------------------------
// loesung + Widmung
// ---------------------------------------------------------------------------

pub fn loesung(id: &str) -> String {
    format!(
        "Für die Übung {id} gibt es hier keine Lösung zum Nachschauen — mit Absicht.\n\n\
         Der Grund ist einfach: Wer die Lösung liest, kann sie danach nicht.\n\
         Wer selbst hinkommt, kann es auch in der Prüfung.\n\n\
         Das hilft dir jetzt weiter:\n\
         1. {WB} check {id}  — der Hinweis sagt dir, was noch fehlt.\n\
         2. Lies die AUFGABE.md noch einmal langsam. Oft steht die Antwort im Text.\n\
         3. Frag jemanden aus deiner Gruppe. Erklären ist die beste Übung.\n\
         4. Frag deinen Trainer oder deine Trainerin. Dafür sind sie da.\n\n\
         Steckst du fest? Das ist normal und gehört zum Lernen dazu."
    )
}

/// Hidden dedication (CLAUDE.md rule 10). Not listed in the help.
pub fn deo_gratias() -> String {
    "Soli Deo gloria.\n\n\
     Gebaut in Dankbarkeit — für Menschen, die noch einmal von vorne anfangen.\n\
     Ad maiorem Dei gloriam."
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_edges() {
        assert_eq!(fortschrittsbalken(0, 8), "[--------------------] 0 %");
        assert_eq!(fortschrittsbalken(8, 8), "[####################] 100 %");
        assert_eq!(fortschrittsbalken(0, 0), "[--------------------] 0 %");
    }

    #[test]
    fn line_plural_is_german() {
        assert_eq!(zeilen(1), "1 Zeile mit Text");
        assert_eq!(zeilen(3), "3 Zeilen mit Text");
        assert_eq!(versuche(1), "1 Versuch");
        assert_eq!(versuche(2), "2 Versuche");
    }

    #[test]
    fn broken_answers_name_the_backslash_trap() {
        // A learner writing a Windows path into a double-quoted TOML value
        // breaks the whole file. The message has to say what to do instead —
        // otherwise every answer check of the exercise fails at once with a
        // parser error nobody can read.
        let escape = detail_text(&Detail::AnswersBroken {
            path: "abgabe/antworten.toml".to_string(),
            problem: "invalid escape sequence expected `b`, `f`, `n`".to_string(),
        });
        assert!(escape.contains("einfache Anführungszeichen"), "{escape}");
        assert!(escape.contains(r"'C:\wb\probe'"), "{escape}");

        // Other parse errors must not get the misleading backslash advice.
        let other = detail_text(&Detail::AnswersBroken {
            path: "abgabe/antworten.toml".to_string(),
            problem: "expected an equals, found a newline".to_string(),
        });
        assert!(!other.contains("Anführungszeichen:"), "{other}");
        assert!(other.contains("schluessel"), "{other}");
    }

    #[test]
    fn markers_are_paired_with_a_word() {
        let symbols = Symbols::set(false);
        let line = check_zeile(&symbols, true, "datei-da");
        assert!(line.contains("erledigt"));
        let line = check_zeile(&symbols, false, "datei-da");
        assert!(line.contains("offen"));
    }

    #[test]
    fn marker_cells_use_the_same_column_width() {
        // Both sets occupy five terminal columns, so rows line up either way.
        assert_eq!(ASCII.cell(ASCII.done), "[ok] ");
        assert_eq!(ASCII.cell(ASCII.open), "[  ] ");
        assert_eq!(UNICODE.cell(UNICODE.done), "✅   ");
        assert_eq!(UNICODE.cell(UNICODE.open), "⬜   ");
    }

    #[test]
    fn hints_never_promise_a_solution() {
        let text = loesung("01-test");
        assert!(text.contains("keine Lösung"));
        assert!(!text.to_lowercase().contains("hier ist die lösung"));
    }
}
