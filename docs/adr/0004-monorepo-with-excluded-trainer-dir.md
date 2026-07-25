# ADR 0004 — One public repo, `trainer/` excluded from the learner ZIP

- Status: accepted
- Date: 2026-07-25
- Milestone: M1
- Deciders: Sebastian Kern

## Context

Four kinds of material belong to Werkbank, with three different audiences:

1. the runner (`runner/`) — developers,
2. exercise content (`uebungen/`) — learners,
3. trainer material (`trainer/`: handbook, authoring guide) — trainers,
4. solutions — trainers only, and never learners.

They change together: a new check type is useless without content that uses it,
and content is useless without a handbook entry. Splitting them across
repositories would mean coordinating releases for what is, in practice, one
change.

## Decision

**One public repository** (`stoicera/werkbank`) holding 1–3, plus **one private
repository** (`werkbank-loesungen`) holding solutions, mirroring exercise IDs.

- `trainer/` lives in the public repo but is **excluded from the learner ZIP**
  by `just package`, together with dotfiles. The ZIP is the security boundary
  that matters in the classroom, not repository visibility.
- Solutions are the one thing that never enters the public repo in any form —
  see ADR 0003.
- Licences split by directory: `runner/` MIT OR Apache-2.0, `uebungen/` and
  `trainer/` CC BY-NC-SA 4.0.
- `dist/MANIFEST.txt` lists exactly what a ZIP contains, so "did the handbook
  leak into the learner package?" is answerable by reading a file.

## Consequences

Positive:

- One commit, one review, one tag can carry runner + content + handbook.
- The repo is public and readable, which is the point: it is credibility for
  teaching work and the basis for later lectures.
- The packaging step is the single place where audience separation happens, so
  it is easy to test and easy to audit.

Negative / accepted costs:

- Exclusion is enforced by the packaging recipe, not by access control. If
  `just package` is wrong, trainer material ships. The manifest exists to make
  that visible, and M3's test protocol checks a built ZIP.
- Contributors see trainer material they may not need. Acceptable: it is not
  secret, only not-for-learners.
- Two repositories still have to be kept in sync by hand for solutions. This is
  a deliberate cost of never having plaintext answers in a public repo.
