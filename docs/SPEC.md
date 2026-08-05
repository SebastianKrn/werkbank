# SPEC — werkbank (English, repo-near)

> Technical spec for the coding agent. Product rationale lives in `PRD.md` (German). When PRD and SPEC conflict, SPEC wins for implementation details, PRD wins for scope.

## 1. Architecture Overview

No server. No accounts. No database. The entire system is:

```
┌─────────────────────────────────────────────────────┐
│  Distribution ZIP  (built from a tag, ADR 0006)     │
│                                                     │
│  werkbank-geraetetechnik/                           │
│  ├── wb.exe / wb            ← static runner binary  │
│  ├── START_HIER.md          ← 1-page German intro   │
│  ├── uebungen/              ← exercise folders      │
│  │   ├── 01-hardware-steckbrief/                    │
│  │   │   ├── AUFGABE.md                             │
│  │   │   ├── exercise.toml  ← metadata + checks     │
│  │   │   ├── material/      ← given files (if any)  │
│  │   │   └── abgabe/        ← learner writes here   │
│  │   └── ...                                        │
│  └── .werkbank/fortschritt.json  ← created by wb    │
└─────────────────────────────────────────────────────┘
         learner runs:  wb check   /  wb status  /  wb bericht
```

Data flow: `wb` discovers exercises by scanning `uebungen/*/exercise.toml` → runs declarative checks against `abgabe/` → prints German feedback → updates `fortschritt.json` → `wb bericht` renders a hand-in summary. That's it. Everything else is content.

**Trade-off note (explicit):** a Rust static binary was chosen over (a) Python scripts — target machines have no guaranteed runtime; (b) Go — team stack is Rust (digitales-nest); (c) plain PowerShell — poor UX for feedback/progress and fragile across execution policies.

**Target environment (confirmed M0, 2026-07-25):** learners work on Windows 11 laptops (32 GB RAM) and run exercises inside **learner-administered Windows 11 / Windows Server VMs** — admin rights available, USB unrestricted, Raspberry Pi optionally available. Consequences: `wb` + exercise ZIP live *inside the VM*; real system captures replace most staged exports; VM snapshots are the didactic "reset button" (document in AUFGABE.md pattern: "Snapshot vor Beginn"). Verify SmartScreen behavior once on a real VM during M3.

## 2. Runner (`wb`) — Rust CLI

Single crate `runner/` (workspace-ready if it grows). Binary name `wb`.

- Rust stable, `clap` (derive) for CLI, `serde` + `toml` for config, `regex`, `sha2` for answer hashing, `encoding_rs` for Windows encoding tolerance (detection is a fixed deterministic order, not statistical — `chardetng` was considered and not used), `serde_json` for `--json`/`bericht.json`/`fortschritt.json`. No async, no network deps. Build targets: `x86_64-pc-windows-msvc` (primary) and `x86_64-unknown-linux-gnu` (dev); macOS is not built today.
- All learner-facing strings live in one `strings_de.rs` module (future i18n by module swap, not a framework).
- Exit codes: 0 = all checks pass, 1 = some fail, 2 = usage/config error. Machine-readable `--json` flag on `check`/`status` (for tests and future integrations).

### Commands

| Command | Behavior |
|---|---|
| `wb` / `wb hilfe` | German help, lists commands with examples. |
| `wb status` | Progress map: per exercise ✅ / 🔨 / ⬜, module progress bar, next suggested exercise. |
| `wb check [ID]` | Run checks for exercise ID, or for the "current" (first not-passed) exercise if omitted. Output per check: pass line or **one hint** (`hint_de`), never the solution. On full pass: short encouragement + pointer to next exercise. |
| `wb erfasse <preset> [ID]` | Convenience capture: runs a **whitelisted** command and writes its output into the exercise's `abgabe/`. Presets fixed in the binary per platform (via `powershell -NoProfile -Command`): `systeminfo`, `ipconfig`, `hardware` (Win32_ComputerSystem/Processor/PhysicalMemory), `firmware` (BiosFirmwareType + PartitionStyle), `datentraeger` (Get-Disk/Get-PhysicalDisk), `spiegel` (Get-VirtualDisk + Get-StoragePool), `bitlocker` (manage-bde -status), `schutz` (Get-MpComputerStatus + Get-NetFirewallProfile), `ordnerliste` (native Rust, `--ordner PFAD` to pick a folder inside the exercise). Full form: `wb erfasse [NAME] [ID] [--ordner PFAD]`; a bare `wb erfasse` lists the presets. Linux equivalents where meaningful. Never executes anything from exercise content. |
| `wb bericht` | Renders `bericht.txt` (German, human-readable) + `bericht.json`: learner alias (asked once, stored locally), per-exercise status, timestamps, attempt counts, integrity hash (SHA-256 over the canonicalized **report** — alias, module, timestamp, summary counters and per-exercise rows — plus a salt) so casual tampering is detectable. Not cryptographically strong — documented as such, and no verifier ships today. |
| `wb loesung <ID>` | Refuses with a friendly German message pointing to the trainer (command exists so learners searching for it get didactics, not silence). |

### Security constraints (binding)

- The runner never executes commands, scripts, or code found in exercise folders. Check types are purely declarative; `command_capture` presets are compiled into the binary.
- No network I/O anywhere in the runner.
- Paths from `exercise.toml` are validated: must resolve inside the exercise folder (no `..`, no absolute paths).

## 3. Exercise Format (topic-agnostic — this is the product)

One folder per exercise. `exercise.toml`:

```toml
[exercise]
id = "01-hardware-steckbrief"
titel = "Hardware-Steckbrief deines PCs"
modul = "geraetetechnik"
schwierigkeit = 1            # 1–3
zeit_minuten = 30
ki_stufe = "ohne"            # "ohne" | "danach" | "frei"  (didactic marker, printed in status/AUFGABE)
lb_relevant = true           # maps to Leistungsbeurteilung topics; shown prominently in wb status
vertiefung = [               # optional external references, never required to pass
  "https://learn.microsoft.com/training/modules/explore-support-diagnostic-tools/",
]

[[check]]
id = "datei-vorhanden"
type = "file_exists"
path = "abgabe/systeminfo.txt"
hint_de = "Führe 'wb erfasse systeminfo' aus — oder speichere die Ausgabe von 'systeminfo' selbst als abgabe/systeminfo.txt."

[[check]]
id = "ram-erkannt"
type = "file_matches"
path = "abgabe/systeminfo.txt"
pattern = '(?i)(Gesamter physischer Speicher|Total Physical Memory)'
hint_de = "Die Datei scheint unvollständig. Hast du die ganze Ausgabe gespeichert?"

[[check]]
id = "frage-speichertyp"
type = "antwort"
file = "abgabe/antworten.toml"
key = "speichertyp"
# sha256(lowercase(trim(answer)) + salt); multiple accepted spellings = multiple hashes
salt = "wb1:01"
expect_hash = ["9f2c…", "a11b…"]
hint_de = "Schau in der Datenträgerverwaltung nach: Ist dein Systemlaufwerk eine SSD oder eine HDD?"
```

### Check types (MVP — exactly these six, resist adding more)

| type | Semantics |
|---|---|
| `file_exists` | Path exists and is non-empty. |
| `file_matches` | Regex against file content; content decoded tolerantly (BOM → BOM-less UTF-16LE → UTF-8 → CP850 fallback; the UTF-16LE heuristic must run *before* UTF-8, because ASCII in UTF-16LE is also valid UTF-8), regex applied case-per-pattern. Files are read up to a fixed cap (8 MiB) so a runaway file in `abgabe/` cannot exhaust memory. |
| `antwort` | Learner answer in `abgabe/antworten.toml` under `key`; normalized (trim, lowercase, collapse whitespace) then salted-SHA-256 compared against `expect_hash` list. |
| `datei_zeilen_min` | File has ≥ N non-empty lines (for free-form deliverables like inventory lists / diagnosis reports). |
| `alle_antworten` | Convenience: every key listed exists in `antworten.toml` **and is non-empty** — a whitespace-only value counts as missing (catches "forgot to fill in"). |
| `werte_gleich` | Two keys in `antworten.toml` must be equal after normalization — e.g. SHA-256 before deletion vs. after restore (the backup-validity proof). |

Any check may carry `stufe = "bonus"` or `stufe = "homelab"` (default `"basis"`). Non-basis checks never block an exercise from counting as passed; `wb status` and `wb bericht` show them separately (Basis ✅ · Bonus 2/3 · Homelab —). This is the whole differentiation mechanism for the heterogeneous group — one exercise, three depths; do not build separate exercise variants.

Free-text deliverables (e.g. the final diagnosis report) are checked for *presence and minimal structure* only — grading stays with the trainer. Do not attempt NLP/LLM grading in MVP.

### Authoring pipeline

- `tools/hash-antwort` (tiny Rust bin or `wb intern hash` hidden subcommand): takes salt + accepted answers, prints `expect_hash` entries. Used by content authors; documented in `trainer/AUTOREN.md`.
- `wb intern lint` validates all `exercise.toml` files (schema, path escapes, regex compile, dangling IDs) **and** the exercise as a deliverable (ADR 0008): `AUFGABE.md` exists and is non-empty, every checked file is named in it, every answer key is discoverable from `AUFGABE.md` or `material/antworten-vorlage.toml`, and every check reads from `abgabe/`. Runs in CI.

## 4. Module: Gerätetechnik (pilot content, German)

Mapped to CompTIA A+ Core 1 (220-1201) domains Hardware (25%) / HW & Network Troubleshooting (28%) and free Microsoft Learn modules as `vertiefung` links. Content 100% original — **never copy from BBRZ material or wissen.raphaellugmayr.at** (IP hygiene, PRD §9).

**LB mapping (input received 2026-07-25):** the pilot group's „Praktische Leistungsüberprüfung" is a 100-minute hands-on exam on a Windows Server 2022 QEMU VM (no TPM): own scenario + files (A1), hardware inspection via CIM/Get-Disk (A2), RAID 1 via diskpart-VHDX + Storage Spaces (A3), SHA-256 integrity (A4), robocopy full/incremental + VSS snapshot (A5), data-loss restore with hash proof (A6), disk-failure simulation + repair (A7), BitLocker password-protector + Defender/Firewall (A8), five reflection questions (R), optional bonus: hash manipulation / SMB share / iSCSI target (A9), mandatory cleanup (−5 points if skipped).

**Exam integrity rules (binding):** the LB PDF and its text never enter this repo (it is Raphael's/BBRZ's exam). Werkbank exercises train the *same competencies with different parametrization* (different sizes, names, drive letters, file counts, wording) — never reproduce exam tasks verbatim, never ship its grading answers. The final Generalprobe is a structurally similar but distinct scenario.

| # | Übung | LB | Kern | Checks (sketch) |
|---|---|---|---|---|
| 01 | Dein Server, deine Firma | A1 | PowerShell-Einstieg: eigenes Szenario, Ordner + Dateien (`New-Item`, `Out-File`, `Get-ChildItem`) | `file_exists` szenario.txt, `datei_zeilen_min`, capture Ordnerliste |
| 02 | Was steckt in der Kiste? | A2 | CIM-Inventur (CPU/RAM/Board), UEFI vs. BIOS, GPT vs. MBR, SSD vs. HDD, „woran erkennst du die VM?" | `wb erfasse hardware/firmware/datentraeger` + `antwort` (Kerne, RAM, Firmware, Partitionsstil, VM-Indiz) |
| 03 | Zwei Platten, ein Spiegel | A3 | diskpart-VHDX ×2, Storage Pool, Mirror, NTFS-Volume | `wb erfasse spiegel` + `file_matches` (Mirror, Healthy) + `antwort` (warum RAID 1) |
| 04 | Fingerabdruck & Backup | A4+A5 | SHA-256, robocopy voll → inkrementell, VSS-Schattenkopie | `file_matches` robocopy-Logs (alle vs. 1 kopiert), `antwort` Hash notiert, `antwort` voll/inkrementell |
| 05 | Daten weg — und zurück | A6 | Löschen, Restore aus Backup, Hash-Beweis byte-genau | `werte_gleich` (hash_vorher = hash_nachher) + capture |
| 06 | Die Platte stirbt | A7 | Dismount-DiskImage, Degraded lesen, Mount + Repair-VirtualDisk | zwei Captures: `file_matches` (Warning/Degraded, Datei lesbar) → (Healthy) |
| 07 | Tresor zu, Tresor auf | A8 | BitLocker Passwort-Protector (ohne TPM), Lock/Unlock, Defender + Firewall zeigen | `wb erfasse bitlocker/schutz` + `file_matches` (Protection On, AES 256) |
| 08 | Generalprobe | alle + R | Eigenes Mini-Szenario end-to-end (andere Parameter als LB), Reflexionsfragen R1–R5, **Aufräumen** | `alle_antworten` (Reflexion), `datei_zeilen_min`, Abschluss-Capture zeigt aufgeräumten Zustand |

Every exercise ends with its own small Aufräumen step + check (the LB deducts 5 points for skipped cleanup — train the habit from exercise 01). Bonus-stufe examples: A9 variants (hash manipulation in 04, SMB share in 08); homelab-stufe: repeat inventory on the Raspberry Pi and compare with the VM.

Each `AUFGABE.md`: goal in one sentence → steps → deliverable definition → `ki_stufe` box → 1 reflection question. Language: German, ~B1, short sentences, no unexplained anglicisms.

`trainer/` (same monorepo, but **excluded from the learner ZIP**): `HANDBUCH_GERAETETECHNIK.md` (session plan, per-exercise pitfalls, timing), `AUTOREN.md`. Solutions live in a separate **private** repo `werkbank-loesungen` (folder-per-exercise mirroring IDs) — never in this repo, not even hashed-out.

## 5. Packaging & Distribution

- The classroom ZIP is produced **only by pushing a `vX.Y.Z[-rcN]` tag** (ADR 0006): `.github/workflows/release.yml` gates on fmt/clippy/tests/content-lint, builds `wb.exe` on windows-latest, then calls `scripts/paket.sh` — the single assembly point, shared by CI and `just`.
- Result `dist/werkbank-geraetetechnik-vX.Y.Z.zip`: runner binaries (win + linux), `START_HIER.md`, `uebungen/`, `VERSION.txt`, `uebungen/LICENSE`, minus `trainer/`, minus dotfiles. Deterministic content listing in `dist/MANIFEST.txt`, checksums in `dist/SHA256SUMS.txt`.
- `just package geraetetechnik` on a dev machine **exits 2** unless `--erlaube-ohne-windows` is passed: the Linux box cannot cross-compile `wb.exe`, and a ZIP without it is classroom-useless. Use the waiver for pipeline testing only.
- `START_HIER.md`: entpacken → Doppelklick/`wb status` → erste Übung. One page, printable (trainer hands it out).
- Releases via GitHub Releases; ZIP is the unit of delivery to the classroom (USB or download — M0 checklist decides).

## 6. Repository Layout

```
werkbank/                      (public repo: SebastianKrn/werkbank)
├── CLAUDE.md
├── README.md                  (English; what/why/quickstart, screenshots)
├── docs/                      (PRD.md, SPEC.md, MILESTONES.md, TESTPROTOKOLL.md, ADRs)
├── runner/                    (Rust crate `wb`)
│   ├── src/…  (cli.rs, checks/, progress.rs, report.rs, capture.rs, strings_de.rs)
│   └── tests/ (integration: fixture exercises under tests/fixtures/)
├── uebungen/geraetetechnik/…  (exercise folders as in §3)
├── trainer/                   (handbook + authoring docs; excluded from learner ZIP)
├── scripts/paket.sh           (learner ZIP assembly — the only copy; ADR 0006)
├── justfile                   (build, lint, test, package)
└── .github/workflows/         (ci.yml, release.yml)

werkbank-loesungen/            (private repo; mirrors exercise IDs)
```

## 7. Security, Privacy, IP

- No telemetry, no network calls, no PII in repo. Learner alias only in local progress + report the learner hands over themselves.
- Content license: see PRD §10 (owner decision; default CC BY-NC-SA 4.0 for `uebungen/` + `trainer/`, MIT OR Apache-2.0 for `runner/`). `LICENSE` files split accordingly.
- No BBRZ-internal content, names of learners, or Wissensdatenbank excerpts anywhere. Example data in `material/` is invented. **The Leistungsbeurteilung PDF is reference input for the authors only — it never enters this repo, and exercises never reproduce its tasks verbatim (re-parametrize; see §4 exam integrity rules).**
- **Faith expression policy (owner decision, PRD §10.7):** learner-facing exercise content in the BBRZ ZIP stays confessionally neutral. Permitted and intended: `SOLI_DEO_GLORIA.md` colophon in repo root, hidden `wb deo-gratias` easter-egg command (prints a short dedication), subtle references in invented sample data (hostnames like `aquinas`, `edith-stein`), saint-themed release names. No scripture quotes, catechism content, or proselytizing text in `AUFGABE.md`, CLI feedback, or trainer handbook.
- Runner threat model: hostile exercise content must not gain execution (see §2 security constraints); learner "cheating" is mitigated (hashed answers, report integrity hash) but explicitly not a hard guarantee — this is a classroom tool, not an exam system.

## 8. Testing (binding)

- `cargo fmt --check`, `clippy -D warnings`, `cargo test` (unit: each check type incl. encoding fallbacks UTF-8/UTF-16LE/CP850, path-escape rejection, answer normalization; integration: run `wb check`/`status`/`bericht` against fixture exercises via `assert_cmd`).
- `wb intern lint` over `uebungen/` in CI (content is code).
- One CI job runs the full learner happy path on `windows-latest` (this is the platform that matters) + `ubuntu-latest`.
- Manual test protocol in `docs/TESTPROTOKOLL.md` for M3: fresh Windows user account in a learner-administered VM (the M0 environment — admin exists inside the VM, and exercises 03–07 need it), execute the START_HIER flow end-to-end.

## 9. Out of Scope (MVP) — restated for the coding agent

No server/API/DB/accounts/web UI. No LLM calls anywhere in the runner. No additional check types beyond §3's six. No second module. No Moodle/LMS export beyond `bericht.txt`/`.json`. No auto-update mechanism. No installer (ZIP only). No English learner content. If a requirement seems to need any of these — stop and ask the owner.
