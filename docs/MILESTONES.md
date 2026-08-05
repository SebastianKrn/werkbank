# MILESTONES — werkbank

> One milestone = one session scope (M1–M3 are Claude-Code sessions; M0 and M4 are human work).
> Never start the next milestone with a red pipeline. Never start M1 before the M0 gate is green.

## M0 — Validation gate (human, no code) — **HARD GATE**

Owner: Raphael (BBRZ side) + Sebastian (product side). Target: ≤ 2 weeks (by 2026-08-08).

- [x] Raphael confirms external practice material may be used in his Gerätetechnik classes. **(2026-07-25: yes, external material allowed.)**
- [x] Classroom constraints checklist: **Windows 11 laptops, 32 GB RAM; per-exercise Windows 11 or Windows Server VMs (learner-administered); USB unrestricted; Raspberry Pi available for experiments.** Group size ~7, heterogeneous (ex-hotel, ex-Altenpflege, some with own homelab).
- [x] Decision recorded: **EXE distribution viable** — exercises run inside learner VMs where learners have admin. Verify SmartScreen behavior once on a real VM in M3, no fallback needed.
- [ ] Pilot group known (~7) — **date window still open**; anchor: upcoming „Leistungsbeurteilung" (LB). Werkbank pilot = LB preparation.
- [ ] 42-friend agrees to be external beta tester (M3).
- [x] **Content input for M2:** LB document received 2026-07-25 („Praktische Leistungsüberprüfung", 100 min, Windows Server 2022 QEMU-VM, A1–A9 + R). Mapping done in SPEC §4. **M2 unblocked.** LB PDF stays out of the repo (exam integrity, SPEC §7).

**Gate result recorded at the bottom of this file. If the EXE path is blocked and no fallback is acceptable → project parks (like eeg-tooling), no code written.**

## M1 — Runner core (Claude-Code session 1) — **done 2026-07-25**

Scope, in order:

1. Scaffold repo per SPEC §6 (runner crate, justfile, CI per SPEC §8, README stub, docs/ copied in).
2. Implement exercise discovery + `exercise.toml` parsing with full validation (schema, path-escape rejection, regex compile) — `wb intern lint`.
3. Implement the six check types (SPEC §3) with encoding-tolerant file reading; unit tests per type incl. UTF-16LE and CP850 fixtures.
4. Implement `wb check`, `wb status`, `wb hilfe` with German output (strings_de.rs), progress file, `--json`.
5. Implement `wb erfasse` presets (systeminfo, ipconfig, datentraeger) with Windows + Linux variants; integration tests via fixture exercises.
6. Implement `wb bericht` (txt + json + integrity hash) and `wb loesung` (didactic refusal).
7. ADRs: 0001 single static binary (vs. scripts/CI), 0002 declarative checks only (no execution of content), 0003 hashed answers, 0004 monorepo with excluded trainer/ dir.

DoD: CI green on windows-latest + ubuntu-latest; `wb check` full happy path against fixture module; a non-technical person can follow START_HIER.md draft on a clean Windows user account.

## M2 — Gerätetechnik content (Claude-Code session 2) — **done 2026-07-26**

1. [x] Write all 8 exercises per SPEC §4 LB mapping (AUFGABE.md German ~B1, exercise.toml with lb_relevant/stufen, per-exercise Aufräumen step; re-parametrized vs. the LB — never exam tasks verbatim). Add LICENSE files per decision: content CC BY-NC-SA 4.0, runner MIT OR Apache-2.0, plus `SOLI_DEO_GLORIA.md` colophon.
2. [x] Generate answer hashes via authoring tool; solutions written to the private `werkbank-loesungen` repo structure (local folder `../werkbank-loesungen`, Sebastian pushes privately).
3. [x] `trainer/HANDBUCH_GERAETETECHNIK.md` (session plan for 4 blocks à ~100 min — the LB's own length, per-exercise pitfalls, timing, what to project) + `trainer/AUTOREN.md`.
4. [x] `START_HIER.md` final + printable one-pager (`trainer/AUSTEILEN_A4.md`).
5. [x] `wb intern lint` green over full module (in CI on both platforms); content reviewed against PRD §7 language rules.

Decisions taken in M2:

- **ADR 0005**: `antwort` hashes only for closed vocabularies; machine-specific values (cores, RAM, firmware type in a QEMU VM) are presence-checked with `alle_antworten`, because a hashed expectation would fail honest work.
- Exercises target **Windows Server 2022 first**, with explicit „Falls du Windows 11 nutzt" notes where commands differ (VSS in 04, BitLocker feature in 07).
- Repo/licence naming aligned with the real remote: **SebastianKrn/werkbank**, copyright Sebastian Kern.
- ADR 0003 corrected: accepted answer spellings live in the private solutions repo, **not** in `trainer/` (that line contradicted CLAUDE.md rule 6).

DoD: a ZIP that passes the full learner flow on a clean machine; Raphael has reviewed exercise list against his curriculum (30-min review call — content fit is his call, not ours).

> **Correction (M3a).** The DoD command was `just package geraetetechnik`; since ADR 0006 that exits 2 on the Linux dev machine without `--erlaube-ohne-windows`, and the classroom ZIP comes from a tag. The Raphael review is still open and is carried as M3b item 3 — M2 is otherwise complete.

Open at the end of M2 (carried into M3):

- [ ] Raphael's 30-min curriculum review of the exercise list.
- [x] Windows binary in the ZIP: **done in M3a.** `.github/workflows/release.yml` builds `wb.exe` on windows-latest and publishes the assembled ZIP from a tag (ADR 0006).
- [ ] Windows-only paths (`manage-bde`, PowerShell presets, UTF-16LE/CP850 captures on a real console) verified only by CI so far, never on a real VM — that is M3 item 1.

## M3 — Pilot packaging & dry runs (mixed)

### M3a — Freeze machinery (Claude-Code session 3) — **done 2026-07-26**

1. [x] `scripts/paket.sh` — packaging as one script shared by `just` and CI. Version in the ZIP filename (SPEC §5), missing `wb.exe` a hard error, tripwire against `trainer/` and solutions in the manifest.
2. [x] `.github/workflows/release.yml` — tag-driven build of both binaries, assembly, verification of the unpacked artifact, publication as a GitHub Release. Version guard against `runner/Cargo.toml`.
3. [x] Packaging smoke test in CI on every run.
4. [x] `docs/TESTPROTOKOLL.md` (SPEC §8, binding) and `trainer/BETA_FEEDBACK.md`.
5. [x] SmartScreen / Mark-of-the-Web instructions in `START_HIER.md` — wording still to be verified against a real Windows 11 (protocol Part A).
6. [x] ADR 0006.

Found while writing the protocol, unfixed by design: **seven of nine capture presets have never been executed anywhere.** Tests run only `ordnerliste` and `ipconfig`; `bitlocker` and `schutz` are `unix: None` and cannot run on the Linux dev box at all. Protocol Part C exists to close this. No pre-emptive fixes were written — guessing at Windows behaviour without evidence is how the protocol gets invalidated before it runs.

### M3a′ — QA sprint before the dry runs (Claude-Code session 4) — **done 2026-08-05**

Unplanned, and the reason M3b now starts against a new release candidate. The
session audited the whole repo along seven independent lenses, verified every
finding adversarially, and fixed what does not need a Windows VM. 53 findings
survived verification; 10 were refuted and are recorded as refuted rather than
silently dropped.

What it changed, by weight:

1. [x] **Two defects a learner would have hit on day one.** Every command the
   runner suggested was a bare `wb check 01`, which PowerShell refuses — and
   `START_HIER.md`'s troubleshooting table then blames the wrong folder. And
   `AUFGABE.md` of exercise 08 told the learner to write a personal answer into
   a hash-checked basis key, so the capstone could not be passed by following
   its own text. The staged solve run could not catch that: it fills in correct
   answers instead of following the task.
2. [x] **Four accepted answers were live in `trainer/AUTOREN.md`**, side by side
   in one table cell, illustrating a rule. The existing tripwire only read
   `intern hash` command arguments. A second one now rejects several accepted
   answers on one line, measured to flag exactly that and nothing else.
3. [x] **`wb intern lint` checks the exercise, not just the schema** (ADR 0008).
   It previously answered "all valid" to a deleted `AUFGABE.md`. Found an
   unearnable bonus check in the test fixture on its first run.
4. [x] **Runner robustness**: a failed progress write no longer eats the check
   feedback; `file_exists` no longer accepts a BOM-only file as content; the
   8 MiB read cap no longer mojibakes a whole file when it splits a character;
   an empty or damaged module is no longer reported as finished; `bestanden_am`
   no longer outlives the pass.
5. [x] **Pipeline**: the version guard no longer lets `v0.1.0-RC2` publish as a
   full release; CI now runs the `wb.exe` that ships; the packaging tripwire now
   catches a symlink; the waived Linux-only ZIP is named so it cannot be
   mistaken for a release; the ZIP is reproducible; the runner's licences ship.
6. [x] **Content**: 19 hints referenced `$a`, a variable undefined in a fresh
   PowerShell window — the learner's evidence file would land in the drive root.
   Cleanup now precedes the final check in exercises 01–07.
7. [x] **Documentation**: ten factual errors corrected, each verified against
   the thing it describes — including a `RELEASE.md` verification command that
   raised a false rule-6 alarm on every clean release.

Tests: 99 → 120. `just ci` now includes the packaging smoke test.

Deliberately **not** done, unchanged from the M3a decision: no speculative fixes
to Windows behaviour (regex patterns against PowerShell output, preset commands,
`manage-bde`). Guessing there destroys the evidence the protocol exists to
collect. Findings of that kind were turned into sharper protocol steps instead.

Still open and still human (carried into M3b): exercise 08's `protokoll.txt`
minimum may be satisfiable by `Start-Transcript`'s own header, and exercise 03's
`spiegel-gesund` may match an unrelated storage pool. Both need a real VM to
settle — Part D and Part C of the protocol.

### M3b — Dry runs (human) — **next**

> **Operator runbook: `docs/M3B_ANLEITUNG.md`** (German). That file is the
> working document — preconditions, order, timings, and the three open
> decisions. This section stays as the milestone record.

Nothing here can be done by a coding session. Each item needs a person, a real
Windows VM, or another human being. They are strictly in order: item 2 is
worthless if item 1 found a blocker.

1. [ ] **Test protocol run.** Sebastian, ~2–3 h, on a fresh Windows Server 2022 VM against **`v0.1.0-rc3`, which has to be cut first** — rc2 predates the QA sprint above and would measure defects that no longer exist, in strings that no longer ship. Build the VM per `docs/VM_WINDOWS_SERVER.md`; instrument is `docs/TESTPROTOKOLL.md` (copy it, fill in the copy, do not commit it). Part C first — it covers the seven capture presets that have never been executed anywhere. Fix everything found, cut `v0.1.0-rc4` per `docs/RELEASE.md`, re-run Parts A and B against it.
2. [ ] **External beta.** 42-friend, solo and remote. Send exactly three things: the release link, `trainer/BETA_FEEDBACK.md`, and the instruction to ask nobody. Answering one question by hand destroys the measurement. Blocked on item 1 being clean.
3. [ ] **Dry-teach.** Sebastian teaches one exercise to Raphael as role play; handbook gaps fixed in `trainer/HANDBUCH_GERAETETECHNIK.md`. Combine with Raphael's outstanding 30-min curriculum review (open since M2).
4. [ ] **Freeze.** Bump `runner/Cargo.toml` to `0.1.0` if it moved, tag `v0.1.0`, verify the published ZIP, print `trainer/AUSTEILEN_A4.md` one-pagers. The tag is the whole freeze procedure — see `docs/RELEASE.md` and ADR 0006.

Carried into M3b from the hardening review (PR #3), still open — all three are
Sebastian's call, written up with recommendations in `docs/M3B_ANLEITUNG.md` §3:

- [ ] Re-salting the leaked answer words (recommendation: don't — re-salting does not un-leak them, and ADR 0003 already accepts brute-forceability).
- [ ] Whether the public `trainer/` handbook may name accepted answers (recommendation: explicit carve-out in CLAUDE.md rule 6; `scripts/paket.sh` already enforces the ZIP boundary mechanically).
- [ ] Code signing / build provenance (recommendation: buy nothing for a seven-learner pilot — no paid path clears SmartScreen without reputation that will never accrue; switch on free GitHub build provenance instead).

Known coverage gap remaining after PR #3: the `wb erfasse ordnerliste` walk is
junction-tested only on Unix (`wb check` **is** covered on windows-latest since
`1a584a0`). Protocol Part F closes it.

Published pre-release: **v0.1.0-rc2** (2026-07-27) — **superseded**, do not test it. The QA sprint of 2026-08-05 changed learner-facing strings and content after it was built. The test object is `v0.1.0-rc3`, to be cut from main per `docs/RELEASE.md` as the first step of M3b. No human has yet run any build on Windows.

DoD: beta tester finished ≥ 6/8 exercises without mechanical help; ZIP frozen ≥ 3 days before pilot date (and the pilot itself completes before the LB date).

## M4 — Pilot execution & measurement (human) — **EXPANSION GATE**

1. Run the pilot in Raphael's class (Sebastian present at least once, ideally as Supplierung).
2. Collect: `wb bericht` files, paper/Forms feedback, trainer observations.
3. Retro against PRD §8 metrics; write `docs/PILOT_RETRO.md`.
4. Decision: expand (second module: Linux or C# — ask the respective BBRZ expert), iterate, or park.

**No second module, no platform features, no monetization work before this gate is evaluated.**

---

## Gate log

| Gate | Date | Result | Notes |
|---|---|---|---|
| M0 | 2026-07-25 | **green** | Permission ✓, environment ✓ (Win11 + learner VMs, EXE viable), LB document ✓ (mapping in SPEC §4), name + licenses decided. Open (non-blocking for M1/M2): pilot date, beta-tester confirmation. |
| M4 | — | open | |
