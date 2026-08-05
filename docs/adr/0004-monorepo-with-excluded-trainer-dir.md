# ADR 0004 — One public repo, `trainer/` excluded from the learner ZIP

- Status: accepted, partly superseded by [ADR 0006](0006-packaging-script-and-tag-driven-release.md)
- Date: 2026-07-25
- Milestone: M1
- Deciders: Sebastian Kern

> **Corrections (M3a, 2026-07-26).** Two statements below were overtaken by
> later decisions and are kept only as a record of what was decided then:
>
> - The repository is `SebastianKrn/werkbank`, not `stoicera/werkbank`. The
>   working name was dropped before the first commit.
> - Exclusion of `trainer/` is enforced by `scripts/paket.sh` — the single
>   assembly point shared by `just` and the release workflow — not by a
>   `just package` recipe. The script also carries a hard tripwire that fails
>   the build if `trainer/`, `loesung` or `lösung` ever reach the ZIP.

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

**One public repository** (written here in July 2026 as `stoicera/werkbank`;
the repo is `SebastianKrn/werkbank` — see Corrections above) holding 1–3, plus
**one private repository** (`werkbank-loesungen`) holding solutions, mirroring
exercise IDs.

- `trainer/` lives in the public repo but is **excluded from the learner ZIP**,
  together with dotfiles. The exclusion was written here as a `just package`
  step and lives today in `scripts/paket.sh` (ADR 0006). The ZIP is the security
  boundary that matters in the classroom, not repository visibility.
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

- Exclusion is enforced by the packaging step, not by access control. If that
  step is wrong, trainer material ships. The manifest exists to make that
  visible, M3's test protocol checks a built ZIP, and `scripts/paket.sh` fails
  the build outright on a forbidden path (the tripwire named in Corrections).
- Contributors see trainer material they may not need. Acceptable: it is not
  secret, only not-for-learners.
- Two repositories still have to be kept in sync by hand for solutions. This is
  a deliberate cost of never having plaintext answers in a public repo.
