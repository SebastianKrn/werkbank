# ADR 0005 — `antwort` hashes only for closed-vocabulary questions

- Status: accepted
- Date: 2026-07-26
- Milestone: M2
- Deciders: Sebastian Kern

## Context

ADR 0003 settled *how* an expected answer is stored (salted SHA-256, plaintext
only in the private solutions repo). Writing the first real module raised the
next question: *which* questions may be checked that way at all.

The Gerätetechnik exercises run inside the learner's own VM. Many interesting
answers therefore differ per learner and per host:

- number of CPU cores and RAM size handed to the QEMU VM,
- firmware type — SeaBIOS gives `Bios`, OVMF gives `Uefi`,
- `MediaType` of a virtual disk (frequently `Unspecified`),
- serial numbers, disk numbers, hash values of self-written files.

A hashed expectation for any of those punishes correct work: the learner reads
their machine honestly, types the truth, and `wb check` calls it wrong. That is
the exact opposite of the product thesis — instant, trustworthy feedback for
someone whose confidence is already thin.

At the same time, presence-only checks everywhere would make the module
toothless: nobody would have to understand *why* a mirror keeps two copies.

## Decision

Split answer questions by whether the correct answer is machine-independent.

- **`antwort` (hashed) is reserved for closed vocabularies** — answers every
  learner must reach identically, one or two words, drawn from the domain
  language: a partition style, a resilience setting, a count of data copies, a
  backup mode, a BitLocker protector, a yes/no. (Deliberately described rather
  than listed: this repo is public, and an enumeration here would be a
  copy-paste answer key — see rule 6 and ADR 0003.)
  Each accepted spelling gets its own `expect_hash` entry (ADR 0003).
- **Machine-specific values are checked with `alle_antworten`** — the key must
  exist and be non-empty. `wb` confirms the learner looked and wrote something
  down; whether "12 GB" is true for that VM is the trainer's call, visible in
  `bericht.txt`.
- **Free-text reasoning** (`warum_raid1`, reflection answers R1–R5) is likewise
  `alle_antworten` only. No NLP, no LLM grading — SPEC §3 and §9.
- **Evidence that the work happened** comes from captures instead: a
  `file_matches` against `abgabe/spiegel.txt` for `(?i)(Healthy|Fehlerfrei)`
  proves the mirror was really built, without pretending to know the learner's
  disk numbers.
- Where a value *must* be compared but cannot be known in advance, use
  `werte_gleich` — the backup proof in exercise 05 compares the learner's own
  SHA-256 before deletion with the one after the restore. The runner never
  learns the value; it only learns that the two agree.

No new check type was needed for any of this, and none may be added (SPEC §9).

## Consequences

Positive:

- Honest work always passes. A learner with 4 cores and a learner with 16 both
  get a green check for the same correct behaviour.
- The hashed answers that remain are genuinely didactic: they force the domain
  term, which is what the Leistungsbeurteilung asks for in its reflection part.
- Locale independence falls out of the same rule. We never hash a value that
  Windows might render as `Healthy` on one machine and `Fehlerfrei` on another;
  those live in `file_matches` patterns as explicit alternations.

Negative / accepted costs:

- `wb` cannot tell a learner that their core count is wrong. Presence-only
  checks are a weaker signal, and a learner can satisfy them with nonsense —
  mitigated by the trainer reading `bericht.txt`, not by the runner.
- Authors carry a judgement call per question ("is this answer the same for
  everyone?"). `trainer/AUTOREN.md` states the rule so the answer does not
  depend on who writes the next module.
- Green Basis checks mean "did the work", not "understood everything".
  The handbook says this explicitly so no trainer over-reads the report.
