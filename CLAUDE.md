# CLAUDE.md — werkbank

> Copy this file into the root of the new repo `stoicera/werkbank`. Copy `PRD.md`, `SPEC.md`, `MILESTONES.md` into `docs/`.

## What this is

**Werkbank** (working title) — a local-first practice layer for Austrian IT retraining (Umschulung). Learners get a ZIP with exercises and a single portable binary `wb`; one command gives instant German feedback. Pilot module: **Gerätetechnik** for BBRZ Wien (trainer: Raphael Lugmayr). The exercise format is topic-agnostic — the format and runner are the product, the module is the first content.

Design thesis (validated at 42 Wels by counterexample): beginners in health-related retraining fail on missing **structure and feedback**, not missing content. Everything serves: small steps, instant feedback, always know where you are.

## Non-negotiable product rules

1. **No server, no accounts, no DB, no web UI, no LLM calls in the runner.** The MVP is a binary + content. If a task seems to need any of these — stop and ask.
2. **The runner never executes exercise-provided commands, scripts, or code.** Checks are declarative (the six types in SPEC §3, no more). `wb erfasse` presets are compiled in.
3. **Zero-install, offline-first.** Target: learner-administered Windows 11 / Windows Server VMs on Windows 11 laptops (confirmed M0). Admin exists inside the VM, but every feature must still survive: no runtime installed, no network. VM snapshots are the didactic reset button.
4. **Learner-facing output is 100% German, simple language (~B1), encouraging, never shaming.** Hints, not solutions. All strings in `strings_de.rs`.
5. **Original content only.** Never copy from BBRZ materials, wissen.raphaellugmayr.at, CompTIA, or Microsoft Learn. Map and link to A+/MS-Learn as `vertiefung`, don't reproduce. Invented example artifacts only. **The Leistungsbeurteilung PDF never enters the repo; exercises train the same competencies with different parametrization — never exam tasks verbatim.**
6. **Solutions never enter this repo** — not in comments, not in tests, not in fixtures. Expected answers exist only as salted hashes. Solutions live in the private repo `werkbank-loesungen`.
7. **No PII, no telemetry.** Learner alias stays local; reports are handed over by the learner, never transmitted.
8. **AI didactics are content, not enforcement.** `ki_stufe` markers and reflection questions; no technical AI-blocking.
9. **The pilot's job is the Leistungsbeurteilung.** Basis-stufe checks map to LB topics (`lb_relevant = true`); bonus/homelab stufen carry everything beyond. One exercise, three depths — never separate exercise variants for strong vs. weak learners.
10. **Faith expression policy (owner: Sebastian, Catholic).** Learner-facing BBRZ content stays confessionally neutral. Intended places for dedication: `SOLI_DEO_GLORIA.md` colophon, hidden `wb deo-gratias` command, subtle sample-data references (hostnames `aquinas`, `edith-stein`), saint-themed release names. Nothing beyond that in AUFGABE.md, CLI output, or handbook.

## Stack

Rust stable, single crate `runner/` (binary `wb`): clap (derive), serde + toml, regex, sha2, encoding_rs/chardetng. No async, no network crates. Cross-compile: windows-msvc (primary), linux-gnu, macos-aarch64. Content: Markdown + TOML under `uebungen/`. Build orchestration: `just`. CI: GitHub Actions (windows-latest is the platform that matters).

## Engineering standards

Group standards apply (Definition of Done, conventional commits, ADRs). Rust: `cargo fmt` + `clippy -D warnings`, unit tests per check type (incl. UTF-16LE/CP850 fixtures), integration tests via `assert_cmd` against fixture exercises, `wb intern lint` over all content in CI. Content is code: a broken `exercise.toml` fails the pipeline.

## Language rules

Code, commits, README, ADRs, technical docs: **English**. Everything a learner or trainer sees (`AUFGABE.md`, CLI output, START_HIER.md, Handbuch, bericht.txt): **German (Austrian, simple language)**. Domain terms stay German: Übung, Abgabe, Fortschritt, Bericht, Vertiefung.

## Way of working

Strictly milestone by milestone (`docs/MILESTONES.md`). **M0 is a hard human gate — if the gate log in MILESTONES.md doesn't say M0 = green, do not scaffold anything.** One milestone = one session. After each: tests green, docs updated, commit, review with Sebastian. When ambiguous: ask, don't invent scope — SPEC §9 has a hard out-of-scope list. The strongest temptation in this project is building platform features; the pilot needs a ZIP that works on a locked-down PC in front of a nervous beginner. Optimize for that person.
