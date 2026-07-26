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

## M1 — Runner core (Claude-Code session 1)

Scope, in order:

1. Scaffold repo per SPEC §6 (runner crate, justfile, CI per SPEC §8, README stub, docs/ copied in).
2. Implement exercise discovery + `exercise.toml` parsing with full validation (schema, path-escape rejection, regex compile) — `wb intern lint`.
3. Implement the five check types (SPEC §3) with encoding-tolerant file reading; unit tests per type incl. UTF-16LE and CP850 fixtures.
4. Implement `wb check`, `wb status`, `wb hilfe` with German output (strings_de.rs), progress file, `--json`.
5. Implement `wb erfasse` presets (systeminfo, ipconfig, datentraeger) with Windows + Linux variants; integration tests via fixture exercises.
6. Implement `wb bericht` (txt + json + integrity hash) and `wb loesung` (didactic refusal).
7. ADRs: 0001 single static binary (vs. scripts/CI), 0002 declarative checks only (no execution of content), 0003 hashed answers, 0004 monorepo with excluded trainer/ dir.

DoD: CI green on windows-latest + ubuntu-latest; `wb check` full happy path against fixture module; a non-technical person can follow START_HIER.md draft on a clean Windows user account.

## M2 — Gerätetechnik content (Claude-Code session 2)

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

DoD: `just package geraetetechnik` produces a ZIP that passes the full learner flow on a clean machine; Raphael has reviewed exercise list against his curriculum (30-min review call — content fit is his call, not ours).

Open at the end of M2 (carried into M3):

- [ ] Raphael's 30-min curriculum review of the exercise list.
- [ ] Windows binary in the ZIP: `just package` on Linux ships only the Linux `wb`. `wb.exe` for the pilot ZIP comes from a windows-latest build in M3 ("Freeze: build final pilot ZIP").
- [ ] Windows-only paths (`manage-bde`, PowerShell presets, UTF-16LE/CP850 captures on a real console) verified only by CI so far, never on a real VM — that is M3 item 1.

## M3 — Pilot packaging & dry runs (mixed)

1. Test protocol run (docs/TESTPROTOKOLL.md) on a fresh Windows account — fix everything found.
2. External beta: 42-friend runs the module solo, remote, no help; collect where he got stuck + quote; fix top findings.
3. Sebastian dry-teaches one exercise to Raphael (role play) — handbook gaps fixed.
4. Freeze: tag v0.1.0, build final pilot ZIP, print one-pagers.

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
