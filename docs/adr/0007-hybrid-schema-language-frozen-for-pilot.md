# ADR 0007 — Hybrid English/German schema language, frozen for the pilot

- Status: accepted
- Date: 2026-07-27 (records a de-facto decision made in M1/M2)
- Milestone: M3 (hardening review, PR #3 item 3)
- Deciders: Sebastian Kern

## Context

`exercise.toml` speaks two languages at once. German keys and check types:
`titel`, `modul`, `schwierigkeit`, `zeit_minuten`, `ki_stufe`, `vertiefung`,
`stufe`, and the check types `antwort`, `alle_antworten`, `werte_gleich`,
`datei_zeilen_min`. English keys and check types: `id`, `type`, `path`,
`file`, `pattern`, `key`/`keys`/`key_a`/`key_b`, `salt`, `expect_hash`,
`lb_relevant`, `min`, and the check types `file_exists`, `file_matches` —
plus the openly hybrid `hint_de`.

The due-diligence review flagged that no ADR covers this: `file_exists`
lives beside `antwort` with no recorded reason, and a future author could
reasonably conclude the mix is an accident to be cleaned up.

It is not an accident, but it did grow rather than get designed. The split
follows a rule that was applied consistently without ever being written
down.

## Decision

**The split rule:** vocabulary that names what a learner or trainer talks
*about* stays German — it is domain language under the project's language
rules (Übung, Abgabe, Stufe, Antwort). Structural plumbing that only the
runner and the toolchain care about follows the English Rust/serde/TOML
ecosystem it is embedded in (`type`, `path`, `pattern`, `expect_hash`).
`hint_de` carries its language in its name because the value, not the key,
is learner-facing.

**The format is frozen for the pilot.** Eight shipped exercises, the test
fixtures, `trainer/AUTOREN.md`, and the private solutions repo's staging
all spell these keys. A rename to either pure English or pure German buys
aesthetic consistency and pays for it with a breaking format change in the
middle of a pilot whose ZIP is already published as a release candidate.
Document, don't rename.

Renaming may be reconsidered **only after the M4 gate**, and only together
with an explicit schema-version marker so old content fails loudly instead
of half-parsing. Until then `deny_unknown_fields` keeps both vocabularies
closed: a misspelled or "translated" key is a lint error, not a silent
no-op.

## Consequences

Positive:

- Authors write domain terms in the language the classroom uses; the
  trainer handbook and `AUTOREN.md` never need to translate a concept back
  and forth.
- Structural keys match every example an author finds when searching for
  TOML or serde documentation.
- The rule is now stated, so the next module's author extends the schema
  deliberately instead of guessing from precedent.

Negative / accepted costs:

- The mix looks undesigned to newcomers until they read this ADR — the
  review proved exactly that.
- Two vocabularies mean two spelling conventions to hold in one head while
  authoring; `wb intern lint` (via `deny_unknown_fields`) is the safety
  net, not human care.
- If post-M4 expansion ever wants a monolingual schema, the migration cost
  grows with every exercise written until then. That cost is accepted
  knowingly: pilot stability outranks schema aesthetics (CLAUDE.md: the
  pilot needs a ZIP that works, not a prettier format).
