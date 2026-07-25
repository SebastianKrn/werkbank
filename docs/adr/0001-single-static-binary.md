# ADR 0001 — One static binary instead of scripts or CI

- Status: accepted
- Date: 2026-07-25
- Milestone: M1
- Deciders: Sebastian Kern

## Context

Werkbank has to run on learner-administered Windows 11 / Windows Server VMs
inside a BBRZ classroom (confirmed in M0). The target person is a beginner in
health-related retraining. The first minute decides whether they keep going.

Constraints that follow from that:

- No guaranteed runtime on the machine. Python, Node and a JVM may all be
  absent, and installing one is a support conversation we cannot afford.
- No network required. Exercises must work in a VM that never gets online.
- No install step, no admin prompt, no execution-policy dialog.
- The unit of delivery is a ZIP handed out on USB or downloaded once.

## Decision

Ship a **single statically-linked Rust binary** `wb` (plus content) in a ZIP.
`x86_64-pc-windows-msvc` is the primary target; Linux and macOS builds exist
for development and for the beta tester.

## Consequences

Positive:

- Unzip and run. No runtime, no installer, no PATH surgery, no admin rights.
- Identical behaviour on every machine — the same binary, the same checks.
- Offline by construction; there is no code path that could reach a network.
- Cross-compilation is a build concern, not a classroom concern.

Negative / accepted costs:

- Rust must be built per target; the release pipeline carries three targets.
- SmartScreen may warn on a freshly downloaded unsigned `.exe`. Learners have
  admin inside their VM, so this is a click, not a blocker — to be verified
  once on a real VM in M3. Code signing stays out of scope for the pilot.
- Binary size (~2 MB) is irrelevant on a USB stick, so `opt-level = "s"` plus
  LTO and stripping is optimisation for tidiness, not necessity.

## Alternatives considered

- **PowerShell scripts.** No runtime problem, but execution policies vary,
  feedback UX is poor, and progress/report handling becomes fragile string
  work. Would also make check definitions executable content — see ADR 0002.
- **Python scripts.** No runtime on the target machines. Rejected in M0.
- **Go.** Would work technically; Rust is the team stack (digitales-nest), and
  there is no second reason to split it.
- **GitHub Classroom / CI-based checking.** Requires accounts, Git literacy and
  network. Git is a learning goal for later, not an entry barrier now.
