# ADR 0002 — Declarative checks only; the runner never executes content

- Status: accepted
- Date: 2026-07-25
- Milestone: M1
- Deciders: Sebastian Kern

## Context

An exercise format that could say "run this command and compare the output"
would be enormously flexible. It would also mean that anyone who can put a file
into `uebungen/` can run code on a learner's machine — and exercise packs are
meant to be shared, forwarded, and eventually written by other trainers.

The threat model is not a targeted attacker. It is a well-meaning trainer
forwarding a ZIP from a colleague, and a learner running it with admin rights
inside a VM that also holds their coursework.

## Decision

Check definitions are **purely declarative**. `exercise.toml` may only select
one of six check types and give it parameters:

`file_exists`, `file_matches`, `antwort`, `datei_zeilen_min`,
`alle_antworten`, `werte_gleich`.

There is no check type that runs a command, and there will not be a seventh
type without a new ADR.

Where running something is genuinely useful — capturing `systeminfo` output —
`wb erfasse` uses a **whitelist compiled into the binary**. Content cannot add
to it, extend it, or pass arguments into it. `ordnerliste` takes a learner-given
path and is therefore implemented natively in Rust rather than as a shell
command, so no learner input is ever interpolated into a command line.

Paths coming from content are validated when the exercise loads (relative, no
`..`, no drive letters) and re-checked against the canonicalised exercise
directory when a check runs, which also catches symlinks pointing outside.

## Consequences

Positive:

- Hostile or careless content cannot gain execution.
- Checks are inspectable: a trainer can read `exercise.toml` and know exactly
  what will be looked at.
- Behaviour is deterministic and testable, which is why `wb intern lint` can
  make content failures a CI concern.

Negative / accepted costs:

- Some checks are impossible to express, and content must be designed around
  the six types. In practice this is a feature: it forces exercises to define a
  concrete, inspectable deliverable instead of a fuzzy one.
- Free-text deliverables can only be checked for presence and minimal
  structure. Grading them stays with the trainer — no NLP, no LLM (see the
  MVP scope in SPEC §9).
- Adding a genuinely new capability means changing the runner and shipping a
  new binary, not just new content.
