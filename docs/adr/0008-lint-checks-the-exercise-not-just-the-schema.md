# ADR 0008 — `wb intern lint` checks the exercise, not just the schema

- Status: accepted
- Date: 2026-08-05
- Milestone: M3b (autonomous QA sprint, before the protocol run)
- Deciders: Sebastian Kern

## Context

CLAUDE.md states the engineering standard as *content is code: a broken
`exercise.toml` fails the pipeline*. Until now `wb intern lint` delivered
exactly that sentence and nothing more — it validated the TOML: schema,
unknown fields, path escapes, regex compilation, duplicate ids, allowed
value sets.

Measured against five realistic authoring mistakes, the lint reported
`8 exercise(s) checked, all valid.` and exit code 0 for every one of them:

| Mutation | Lint said |
|---|---|
| Check path renamed so `AUFGABE.md` names a different file | valid |
| `hint_de` written in English | valid |
| Answer key renamed, template not updated | valid |
| `material/antworten-vorlage.toml` deleted | valid |
| **`AUFGABE.md` deleted entirely** | valid |

The last row is the shape of the problem. An exercise with no task text
passes every rule the runner had, ships in the ZIP, and hands a learner a
folder to be graded on with nothing to read.

These are not hypothetical. The rules written below found two live defects on
their first run: the demo fixture's bonus check `lieblingsbefehl` required a
key the task text never named — unearnable by construction — and the same
class of drift is what Raphael's pending curriculum review will produce most
of, because editing exercise text and editing checks are two different
motions.

The failure mode is specific and expensive: **the learner does exactly what
the task says and still sees red.** Everything else in this product is built
to prevent that feeling. A beginner who is told they are wrong while being
right does not conclude the tool is broken; they conclude they are.

## Decision

`wb intern lint` gains a second stage, `runner/src/content.rs`, that asks
whether an exercise is fit to hand to a person. Four rules, chosen because
each one describes a mistake that costs a learner time they cannot get back:

1. `AUFGABE.md` exists and is not empty.
2. Every file a check reads is named in `AUFGABE.md`.
3. Every answer key a check requires appears in `AUFGABE.md` **or** in
   `material/antworten-vorlage.toml` — both content styles are legitimate,
   what matters is that the learner can discover the name.
4. Every check reads from `abgabe/`. A check on shipped material grades work
   nobody did.

**The two stages stay separate on purpose.** `exercise::load` decides whether
the *runner* can run an exercise, and it keeps saying yes to one whose
`AUFGABE.md` is missing: a learner mid-module must never be locked out of
`wb check` by a content problem they cannot fix. `content::audit` decides
whether an *author* may ship it, and only `wb intern lint` — which runs in
CI, not in a classroom — calls it.

Rule 2 matches on the file name, not the full path, and rule 3 on a plain
substring. Both are deliberately loose: the rule has to survive an author
writing `` `notiz.txt` `` in prose while the check says `abgabe/notiz.txt`.
A rule that cries wolf gets switched off, and a lint that is switched off
protects nobody.

## Consequences

Positive:

- The mistake class that hurts most is now mechanically impossible to ship.
- The invariants held across all eight pilot exercises before the rules
  existed, so this records and defends the standard the content already met
  rather than imposing a new one.
- Content review after Raphael's curriculum pass gets cheaper: the drift
  between task text and checks is caught by CI instead of by a learner.

Negative / accepted costs:

- Four more ways for CI to go red on a content edit. That is the point, but
  it does mean an author renaming a file must touch both places — the error
  message names both.
- Substring matching cannot tell a mention from an instruction. An
  `AUFGABE.md` that names `notiz.txt` only in a "do not create this" sentence
  would satisfy rule 2. Catching that needs a human, and the trainer handbook
  is where that review belongs.
- The rules encode one content shape (task text + `abgabe/` + optional answer
  template). A future module with a different shape has to revisit them; the
  fixtures under `runner/tests/fixtures/ungueltig/` document what each rule is
  worth so that conversation starts from evidence.

## Alternatives considered

**Leave it to review.** Rejected: the review that would have caught the
`lieblingsbefehl` defect had already happened, twice, and did not.

**A stricter rule — checks must be named in `AUFGABE.md` at the exact path.**
Rejected: it fails on the current, correct content, which quite reasonably
writes `` `antworten.toml` `` in one place and `abgabe/antworten.toml` in
another.

**Fold the rules into `exercise::load`.** Rejected: it would make a missing
`AUFGABE.md` break `wb check` for the learner, which turns an author's
mistake into a locked door in the classroom. Wrong blast radius.
