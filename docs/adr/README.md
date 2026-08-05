# Architecture Decision Records

Decisions that would be expensive to reverse, with the trade-offs that were
accepted. New ADRs get the next number; a superseded ADR is marked, never
deleted.

| # | Decision | Status |
|---|---|---|
| [0001](0001-single-static-binary.md) | One static binary instead of scripts or CI | accepted |
| [0002](0002-declarative-checks-only.md) | Declarative checks only; the runner never executes content | accepted |
| [0003](0003-hashed-answers.md) | Expected answers ship as salted hashes only | accepted |
| [0004](0004-monorepo-with-excluded-trainer-dir.md) | One public repo, `trainer/` excluded from the learner ZIP | accepted, partly superseded by [0006](0006-packaging-script-and-tag-driven-release.md) |
| [0005](0005-antwort-hashes-only-for-closed-vocabulary.md) | Hashed answers only for closed vocabularies; machine-specific values are presence-checked | accepted |
| [0006](0006-packaging-script-and-tag-driven-release.md) | One packaging script shared by `just` and CI; the pilot ZIP is built from a tag | accepted |
| [0007](0007-hybrid-schema-language-frozen-for-pilot.md) | The exercise schema mixes English structural keys with German domain keys; frozen for the pilot | accepted |
| [0008](0008-lint-checks-the-exercise-not-just-the-schema.md) | `wb intern lint` also checks that an exercise is fit to hand to a person | accepted |
