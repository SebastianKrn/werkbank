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

Milestone **M3a — freeze machinery — done (2026-07-26)**. The runner is done (M1), the pilot module
ships eight exercises under `uebungen/geraetetechnik/` (M2), and a tag now
produces a classroom-ready ZIP containing `wb.exe`. Next is M3b, which needs a
human and a real Windows VM: the manual test protocol
(`docs/TESTPROTOKOLL.md`), an external beta, then the pilot freeze.
See `docs/MILESTONES.md`.

## The pilot module

Eight exercises preparing a BBRZ Wien retraining group for their practical
assessment. Each one trains the same competency as an exam task, deliberately
re-parametrised — never the exam task itself (`docs/SPEC.md` §4).

| # | Übung | Competency | min |
|---|---|---|---|
| 01 | Dein Server, deine Firma | PowerShell basics: folders, files, listings | 30 |
| 02 | Was steckt in der Kiste? | CIM inventory, UEFI/BIOS, GPT/MBR, spotting a VM | 30 |
| 03 | Zwei Platten, ein Spiegel | diskpart VHDX, storage pool, mirror, NTFS | 45 |
| 04 | Fingerabdruck & Backup | SHA-256, robocopy full/incremental, VSS | 40 |
| 05 | Daten weg — und zurück | delete, restore, prove it with a hash | 35 |
| 06 | Die Platte stirbt | simulate disk failure, read Degraded, repair | 40 |
| 07 | Tresor zu, Tresor auf | BitLocker without TPM, Defender, firewall | 35 |
| 08 | Generalprobe | own scenario end to end, reflection, teardown | 60 |

Exercises 03–07 build on each other; 08 tears the whole stack down and proves it.
Every exercise ends with its own cleanup step and check — the assessment deducts
points for skipped cleanup, so the habit starts in exercise 01.

Trainer material (session plan, pitfalls, authoring guide, printable handout)
lives in `trainer/` and never reaches the learner ZIP.

## How it looks

```
$ wb status

Werkbank — Modul geraetetechnik

  ✅   01-dein-server-deine-firma   Dein Server, deine Firma   LB  Basis 5/5 · Bonus 1/1 · Homelab 1/1
  🔨   02-was-steckt-in-der-kiste   Was steckt in der Kiste?   LB  Basis 4/7 · Bonus 0/1 · Homelab 0/1
  ⬜   03-zwei-platten-ein-spiegel  Zwei Platten, ein Spiegel  LB  Basis 0/9 · Bonus 0/1 · Homelab 0/1
  ⬜   04-fingerabdruck-und-backup  Fingerabdruck & Backup     LB  Basis 0/7 · Bonus 0/2 · Homelab 0/1
  ⬜   05-daten-weg-und-zurueck     Daten weg — und zurück     LB  Basis 0/6 · Bonus 0/1 · Homelab 0/1
  ⬜   06-die-platte-stirbt         Die Platte stirbt          LB  Basis 0/7 · Bonus 0/1 · Homelab 0/1
  ⬜   07-tresor-zu-tresor-auf      Tresor zu, Tresor auf      LB  Basis 0/8 · Bonus 0/2 · Homelab 0/1
  ⬜   08-generalprobe              Generalprobe               LB  Basis 0/7 · Bonus 0/1 · Homelab 0/1

Fortschritt: 1 von 8 Übungen bestanden
Davon prüfungsrelevant (LB): 1 von 8 bestanden
[##------------------] 12 %

Dein nächster Schritt:  wb check 02-was-steckt-in-der-kiste
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

## Releasing

The classroom ZIP is built from a tag, because that is the only place a Windows
binary exists — the development machine is Linux and does not cross-compile
`wb.exe` (ADR 0006).

```sh
git tag v0.1.0-rc1 && git push origin v0.1.0-rc1
```

That builds both binaries, assembles the ZIP through `scripts/paket.sh`, unpacks
the result and runs it, then publishes a GitHub Release. Tags containing `-rc`
publish as pre-releases. The tag must agree with the version in
`runner/Cargo.toml`, or the pipeline refuses.

`just package geraetetechnik --erlaube-ohne-windows` builds a Linux-only ZIP for
local testing. It is useless in a classroom, which is why the waiver has to be
spelled out.

**Never hand-assemble a ZIP.** `scripts/paket.sh` is the only copy of the rules
for what must and must not reach a learner — a second copy is one that will
eventually disagree.

Before any ZIP goes to a class, run `docs/TESTPROTOKOLL.md` on a real Windows
VM. CI proves the archive is well-formed; it cannot prove that `manage-bde`
prints what we think it prints.

## Licence

Runner (`runner/`): MIT OR Apache-2.0 — `LICENSE-MIT`, `LICENSE-APACHE`.
Exercise content and trainer material: CC BY-NC-SA 4.0 — `uebungen/LICENSE`,
`trainer/LICENSE`. Copyright Sebastian Kern.

Solutions are not in this repository and never will be: expected answers exist
only as salted hashes (ADR 0003), plaintext lives in the private
`werkbank-loesungen` repo. Hashes cover closed-vocabulary answers only —
machine-specific values are presence-checked, because a hashed expectation on
"how many cores does your VM have" would fail honest work (ADR 0005).
