# Architecture Decision Records

Decisions that would be expensive to reverse, with the trade-offs that were
accepted. New ADRs get the next number; a superseded ADR is marked, never
deleted.

| # | Decision | Status |
|---|---|---|
| [0001](0001-single-static-binary.md) | One static binary instead of scripts or CI | accepted |
| [0002](0002-declarative-checks-only.md) | Declarative checks only; the runner never executes content | accepted |
| [0003](0003-hashed-answers.md) | Expected answers ship as salted hashes only | accepted |
| [0004](0004-monorepo-with-excluded-trainer-dir.md) | One public repo, `trainer/` excluded from the learner ZIP | accepted |
