# ADR 0006 — One packaging script, and releases built from a tag

- Status: accepted
- Date: 2026-07-26
- Milestone: M3
- Deciders: Sebastian Kern

## Context

At the end of M2 the project could not produce the thing it exists to deliver: a
ZIP that runs in the classroom. Two facts collided.

The development machine is Linux and does not cross-compile `wb.exe`. `just
package` therefore assembled a Linux-only archive and printed a warning about
the missing Windows binary. A warning is exactly what gets scrolled past on
freeze day, and the result would have been a ZIP that is useless in front of a
class — discovered at the worst possible moment.

The assembly rules also lived inline in the `justfile`. CI deliberately does not
use `just`, so that a missing tool can never be the reason a pipeline is red
(the comment at the top of the `justfile` says so). Any pipeline that wanted to
build a ZIP would have had to reimplement the rules — and a second copy of
"which files must never reach a learner" is a copy that will eventually disagree
with the first one. One of those rules is CLAUDE.md rule 6, where disagreement
means shipping solutions to learners.

## Decision

**Packaging lives in `scripts/paket.sh`.** It is the only place that knows how a
learner ZIP is assembled. The `justfile` recipe and the release workflow both
call it; neither reimplements anything.

**The pilot ZIP is built from a tag** by `.github/workflows/release.yml`:
`windows-latest` and `ubuntu-latest` each build a release binary, a packaging
job assembles the ZIP from both, and a release job publishes it as a GitHub
Release (SPEC §5). Tags containing `-rc` publish as pre-releases.

Three guards sit on that path:

1. **A missing `wb.exe` is a hard error** (exit 2), not a warning. Local
   testing waives it explicitly with `--erlaube-ohne-windows`.
2. **A tripwire** (exit 3) fails the build if `trainer/` or anything
   solution-shaped appears in the manifest. Rule 6 gets a mechanical guard
   instead of relying on a careful author.
3. **A version guard** rejects a tag that disagrees with `runner/Cargo.toml`.
   Release candidates may share the version they are candidates for, so
   `v0.1.0-rc1` matches `0.1.0`.

The release pipeline then **unpacks the archive it just built and runs it** —
`wb.exe` present, `wb status --json` parses and reports eight exercises,
`wb intern lint` green. Verifying the source tree would prove nothing about the
artifact a learner receives.

The extracted folder keeps its unversioned name, `werkbank-geraetetechnik`.
Only the ZIP filename carries the version. `START_HIER.md` tells learners to
`cd C:\werkbank-geraetetechnik`, and that instruction must survive every
release.

## Consequences

- A classroom-ready ZIP exists, which was the blocking gap for all of M3.
- The freeze becomes one command: tag, push, download.
- Every pull request builds a ZIP and asserts its contents, so packaging bugs
  surface in review rather than on freeze day. That check caught a real one: the
  `abgabe/` folders contain only `.gitkeep`, which packaging strips, so they
  survive as zip directory entries alone. A learner without `abgabe/` cannot
  hand anything in, and the staging tree looks perfectly healthy — only the
  archive shows it.
- `scripts/paket.sh` is bash and runs on Linux and macOS, not on Windows. This
  is acceptable: packaging is a maintainer task, and the maintainer machine is
  Linux. Should that change, the script is small enough to port.
- Releases are public, like the repository. The content is CC BY-NC-SA, answers
  are hashed, solutions live in the private repository — nothing in the ZIP is
  secret. Exam integrity rests on re-parametrised exercises (SPEC §7), not on
  the ZIP being hard to obtain.

## Alternatives rejected

**Packaging as a `wb` subcommand.** Tempting — it would run everywhere the
runner runs, and it could be unit-tested in Rust. Rejected because it grows the
runner into a build tool. The runner's job is to give a learner feedback; SPEC
§9 warns specifically against platform features, and the binary a learner
double-clicks should not carry code that only a maintainer ever runs.

**Install `just` in CI and call the recipe.** Would have kept one source of
truth with less churn, but it contradicts the standing rule that CI must not
depend on `just`. A shell script the pipeline runs directly costs nothing and
keeps that rule intact.

**Build the ZIP by hand from downloaded CI artifacts.** What M2 left behind. It
works exactly as long as the person doing it remembers every exclusion rule at
the moment they are tired. Freeze day is not that moment.
