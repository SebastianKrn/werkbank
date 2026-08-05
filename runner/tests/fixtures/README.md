# Test fixtures

Invented mini-content for the test suite. Nothing here is teaching material and
nothing here is copied from anywhere.

## `modul-demo/`

A tiny, valid module with three exercises that between them use all six check
types. The integration tests copy it into a temporary directory before running
`wb`, so the tests never write into the repository.

**No solutions live here** (CLAUDE.md rule 6). The one `antwort` check in
`02-antworten-ueben` carries only salted hashes; the accepted plaintext is not
in this repository, and the integration tests expect that check to stay open.
Where a test needs a passing `antwort`, it computes the hash at runtime from
its own throwaway string (see `src/checks/tests.rs`).

## `ungueltig/`

One folder per authoring mistake that must fail `wb intern lint`:

| Folder | Mistake |
|---|---|
| `pfad-escape` | Check path leaves the exercise folder |
| `kaputte-regex` | `pattern` does not compile |
| `unbekanntes-feld` | Invented field in `[exercise]` |
| `feld-passt-nicht` | Field that has no meaning for that check type |
| `falsche-ki-stufe` | Values outside the allowed sets |
| `ohne-basis-check` | Only optional checks — would pass without work |
| `id-passt-nicht` | `id` does not match the folder name |
| `windows-geraetename` | Path uses a reserved Windows device name |

The next four are schema-valid and still unfit to hand to a person. They cover
`src/content.rs`, which asks the wider question `exercise::load` deliberately
does not ask (see that module's header):

| Folder | Mistake |
|---|---|
| `ohne-aufgabe` | No `AUFGABE.md` — nothing for the learner to read |
| `datei-nicht-erklaert` | Check and task text disagree about the filename |
| `schluessel-nicht-erklaert` | Answer key the learner is never shown |
| `liest-ausserhalb-abgabe` | Check grades a shipped file, not the learner's work |

These deliberately stay outside `modul-demo/`, so the content lint in CI runs
against valid content only.
