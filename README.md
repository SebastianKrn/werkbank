# Werkbank

A local-first practice layer for Austrian IT retraining (*Umschulung*).

Learners get a ZIP with exercises and one portable binary, `wb`. They do the
work, type `wb check`, and get instant feedback in German — a hint, never the
solution. No install, no account, no server, no network.

> Learner-facing content and CLI output are German. Code, commits and docs are
> English. See `CLAUDE.md` for the full working rules.

## Why

People retraining into IT for health reasons do not fail because content is
missing — content is free and abundant. They fail because structure and
feedback are missing: no clear next step, no immediate answer, no "you are
here". Werkbank supplies exactly that, next to whatever course already exists.

The exercise format and the runner are the product. The first module
(*Gerätetechnik*, a pilot at BBRZ Wien) is the first content.

## Status

Milestone **M1 — runner core**. The runner works against fixture exercises;
the Gerätetechnik module is milestone M2. See `docs/MILESTONES.md`.

## How it looks

```
$ wb status

Werkbank — Modul demo

  ✅   01-erste-schritte    Deine erste Notiz      LB  Basis 2/2 · Bonus 0/1
  🔨   02-antworten-ueben   Fragen beantworten     LB  Basis 1/2 · Bonus 0/1
  ⬜   03-ausgabe-erfassen  Eine Ausgabe erfassen      Basis 0/2 · Homelab 0/1

Fortschritt: 1 von 3 Übungen bestanden
Davon prüfungsrelevant (LB): 1 von 2 bestanden
[######--------------] 33 %

Dein nächster Schritt:  wb check 02-antworten-ueben
```

Add `--ascii` if a console renders the symbols poorly.

## Commands

| Command | What it does |
|---|---|
| `wb status` | Progress across all exercises, plus the next step. |
| `wb check [ID]` | Check one exercise. Exit 0 = passed, 1 = still open. |
| `wb erfasse <name> [ID]` | Write system output into the exercise's `abgabe/`. |
| `wb bericht` | Write `bericht.txt` + `bericht.json` for hand-in. |
| `wb loesung <ID>` | Explains why there are no solutions to look up. |
| `wb hilfe` | German help. |

Developer/author tooling (hidden from learners):

| Command | What it does |
|---|---|
| `wb intern lint [PFAD]` | Validate every `exercise.toml` under a folder. Runs in CI. |
| `wb intern hash --salt <SALT> <ANTWORT>...` | Turn accepted answers into `expect_hash` entries. |

## Exercise format

One folder per exercise, one `exercise.toml`. Writing an exercise never
requires touching the runner.

```toml
[exercise]
id = "01-erste-schritte"     # must equal the folder name
titel = "Deine erste Notiz"
modul = "demo"
schwierigkeit = 1            # 1-3
zeit_minuten = 10
ki_stufe = "ohne"            # ohne | danach | frei
lb_relevant = true           # counts towards the Leistungsbeurteilung
vertiefung = ["https://…"]   # optional, never required to pass

[[check]]
id = "notiz-vorhanden"
type = "file_exists"
path = "abgabe/notiz.txt"
hint_de = "Lege im Ordner \"abgabe\" eine Datei notiz.txt an."
```

Six check types, and deliberately no more:

| type | Semantics |
|---|---|
| `file_exists` | Path exists and is non-empty. |
| `file_matches` | Regex against file content. |
| `antwort` | Answer in `abgabe/antworten.toml`, compared as a salted SHA-256. |
| `datei_zeilen_min` | File has at least N non-empty lines. |
| `alle_antworten` | Every listed key is present and filled in. |
| `werte_gleich` | Two answers are equal after normalisation. |

Every check may carry `stufe = "bonus"` or `stufe = "homelab"` (default
`basis`). **Only basis checks decide whether an exercise is passed** — one
exercise, three depths, so a mixed group works from the same material.

## Design constraints (binding)

- **No execution of content.** Checks are declarative. `wb erfasse` presets are
  compiled into the binary; `ordnerliste` is implemented in Rust so no learner
  input ever reaches a shell.
- **No network, anywhere.** No telemetry, no accounts, no LLM calls.
- **Paths stay inside the exercise.** Validated when loading, re-checked
  against symlinks when running.
- **Solutions never live in this repo.** Expected answers exist only as salted
  hashes; plaintext lives in the private `werkbank-loesungen` repository.
- **Windows encodings are tolerated.** Files are decoded UTF-8 → UTF-16LE →
  CP850, deterministically, so `systeminfo > datei.txt` on a German console
  works.

Decisions with their trade-offs are recorded in `docs/adr/`.

## Development

```sh
just build          # or: cargo build --manifest-path runner/Cargo.toml
just test           # unit + integration tests
just lint           # fmt --check + clippy -D warnings
just lint-inhalt    # validate exercise content
just ci             # everything the pipeline checks
```

Requires stable Rust. `just` is convenience only — CI runs the same cargo
commands directly.

## Licence

Runner (`runner/`): MIT OR Apache-2.0. Exercise content and trainer material:
CC BY-NC-SA 4.0. Licence files land with the first content module (M2).
