# ADR 0003 — Expected answers ship as salted hashes only

- Status: accepted
- Date: 2026-07-25
- Milestone: M1
- Deciders: Sebastian Kern

## Context

Exercises ask questions whose answers can be checked mechanically ("Is the
system drive an SSD or an HDD?", "UEFI or BIOS?"). The learner writes them into
`abgabe/antworten.toml`.

The exercise pack lives on the learner's own machine. Anything stored in plain
text in it can be read by anyone who opens the file — and an exercise whose
answer can be read out of the exercise is worth nothing. At the same time this
is a classroom tool preparing people for an exam, not an exam system: the goal
is to remove the *casual* shortcut, not to win an arms race against a
determined learner who is, after all, only cheating themselves.

## Decision

`exercise.toml` stores **salted SHA-256 hashes**, never plaintext:

```
expect_hash = sha256(normalize(answer) + salt)
normalize   = trim -> lowercase -> collapse inner whitespace
```

- Every accepted spelling gets its own hash entry, so "SSD" and
  "solid state disk" can both be correct. German is unforgiving here: `ß` and
  `ss` lowercase differently, so both spellings need their own entry.
- The salt is per-exercise (`"wb1:01"`), which stops identical answers in
  different exercises from producing identical hashes.
- `wb intern hash --salt <SALT> <ANTWORT>...` generates the entries. It prints
  only hashes — the plaintext never reaches stdout, so it cannot be pasted into
  this repository by accident.
- Plaintext solutions live in the separate, private `werkbank-loesungen`
  repository. Not in comments, not in tests, not in fixtures (CLAUDE.md rule 6).
- The same rule governs the report: `bericht.txt` carries a SHA-256 over the
  canonicalised progress plus a compiled-in salt, and says in plain German that
  it detects casual editing and nothing more.

## Consequences

Positive:

- Reading the exercise pack does not reveal answers.
- Feedback stays useful without leaking: a wrong `antwort` says *which key* is
  wrong, never what was expected.
- Answer normalisation absorbs the harmless variation (case, stray spaces)
  that would otherwise frustrate a beginner who is actually right.

Negative / accepted costs:

- Not cryptographically meaningful protection. The salt ships with the
  exercise, and short answers from a small answer space are brute-forceable by
  anyone who wants to. Documented as such, and deliberately not fought.
- Authors cannot read their own expected answers back out of the repository.
  This is why the private solutions repo is not optional.
- A typo in an accepted answer is invisible until someone tries it — content
  review has to cover the answer list, and the accepted spellings are documented
  per exercise in the private solutions repo.

  *Correction (M2, 2026-07-26):* this line originally said the trainer handbook
  carries the accepted spellings. It must not — `trainer/` lives in this public
  repo, and CLAUDE.md rule 6 / SPEC §4 keep plaintext answers out of it entirely.
  The spellings live in `werkbank-loesungen/<modul>/<id>/LOESUNG.md`;
  `trainer/HANDBUCH_GERAETETECHNIK.md` points there and tells the trainer to
  award the point when a correct spelling is missing, then report it so the hash
  can be added.
