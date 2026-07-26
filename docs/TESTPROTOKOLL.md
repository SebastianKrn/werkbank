# TESTPROTOKOLL — manual verification before a pilot freeze

Binding per SPEC §8. Run this on a real Windows VM before any ZIP goes to a
classroom. CI proves that the code compiles, that the content validates and that
the archive is assembled correctly. It cannot prove that `manage-bde` prints what
we think it prints, that a German console does not mangle umlauts, or that a
beginner gets past SmartScreen.

**How to use:** copy this file to `pruefung-YYYY-MM-DD.md` outside the repo and
fill in the copy. This file stays blank — it is the instrument, not the record.
Do not commit filled-in protocols (they contain machine details).

**Who:** Sebastian. Roughly 2–3 hours including one full exercise pass.

**Rule while testing:** you are not allowed to help yourself. If a step needs
knowledge that is not in `START_HIER.md` or the exercise text, that is a
finding — write it down instead of working around it.

---

## Why this document exists in this shape

Two facts drive the priorities below.

**Seven of nine capture presets have never been executed.** The integration
tests run only `ordnerliste` and `ipconfig`. The command strings for
`systeminfo`, `hardware`, `firmware`, `datentraeger`, `spiegel`, `bitlocker` and
`schutz` have never run on any machine in any pipeline. `bitlocker` and `schutz`
are marked `unix: None` in `runner/src/capture.rs` and *cannot* run on the Linux
development box. Part C is therefore the highest-value part of this protocol.

**Encoding is untested against a real console.** The unit tests cover UTF-16LE
and CP850 through fixtures. A German Windows console piping PowerShell output
into a Rust process is a different thing, and umlauts are where it shows.

---

## Environment

| | |
|---|---|
| Host | Windows 11 laptop, 32 GB RAM (the classroom machine per M0) |
| Guest | Windows Server 2022 VM, learner-administered — the LB's own environment |
| Second guest | Windows 11 VM, for the „Falls du Windows 11 nutzt" notes in 04 and 07 |
| User account | A **freshly created** account. Not your developer account. |
| Network | Disconnect after downloading the ZIP. Everything else must work offline. |

**Snapshot before you start**, and after Part B. Exercises 03–07 build one
construction site; you will want to jump back.

- [ ] VM prepared, snapshot `vor-test` taken
- [ ] ZIP downloaded from the GitHub Release page (not copied from the repo —
      the download path is what produces the Mark-of-the-Web)

---

## Part A — Delivery and SmartScreen

This closes the M0 open item ("verify SmartScreen behavior once on a real VM").
`wb.exe` is unsigned and always will be for the pilot: no code-signing
certificate, no fallback (M0 decision). So the only question is whether the
instructions in `START_HIER.md` actually get a beginner through.

| # | Step | Expected | Result |
|---|---|---|---|
| A1 | Look at the downloaded ZIP's properties | An „Zulassen" / „Entsperren" checkbox is present at the bottom | |
| A2 | Follow `START_HIER.md` step 1 exactly: unblock, *then* extract to `C:\` | Folder `C:\werkbank-geraetetechnik` exists | |
| A3 | Run `.\wb status` | Either no warning, or the blue „Der Computer wurde durch Windows geschützt" dialog | |
| A4 | If the dialog appears: „Weitere Informationen" → „Trotzdem ausführen" | `wb` runs. Note the **exact German wording** of the dialog | |
| A5 | **Counter-test:** re-extract without unblocking first | Confirms the second table row in `START_HIER.md` is true. If Windows does *not* re-prompt per file, that row is wrong and must be deleted | |

Record the exact dialog wording — `START_HIER.md` quotes it, and a quote that
does not match what is on screen is worse than no quote.

- [ ] A1–A5 done, wording in `START_HIER.md` corrected if needed

---

## Part B — Cold start

| # | Command | Expected | Result |
|---|---|---|---|
| B1 | `.\wb status` | All eight exercises, `Fortschritt: 0 von 8`, next step named | |
| B2 | Umlaut check in that output | `Zwei Platten, ein Spiegel`, `Daten weg — und zurück` — umlauts and the em dash render correctly | |
| B3 | `.\wb status --ascii` | Same content, no box-drawing symbols | |
| B4 | `.\wb hilfe` | Completely German, no English leaking through | |
| B5 | `.\wb check 01` before doing anything | Fails kindly, names what is missing, gives a hint and **no solution** | |
| B6 | `.\wb loesung 01` | Didactic refusal, points somewhere useful | |
| B7 | `type VERSION.txt` | Matches the Release tag you downloaded | |

If B2 shows `Ã¼` or boxes: that is a finding, not a cosmetic detail. A learner
reading mangled German loses trust in the tool immediately.

- [ ] B1–B7 done

---

## Part C — Capture presets (highest risk)

Run every preset once, against exercise 01, and **open each produced file**.
A preset that exits 0 while writing an error message into the capture file is
the failure mode to hunt for here.

```powershell
.\wb erfasse systeminfo 01
.\wb erfasse ipconfig 01
.\wb erfasse hardware 01
.\wb erfasse firmware 01
.\wb erfasse datentraeger 01
.\wb erfasse spiegel 01
.\wb erfasse bitlocker 01
.\wb erfasse schutz 01
.\wb erfasse ordnerliste 01
```

| Preset | Underlying command | Watch for | Result |
|---|---|---|---|
| `systeminfo` | `systeminfo` | Console codepage; German field names with umlauts | |
| `ipconfig` | `ipconfig /all` | Already covered by CI, should be boring | |
| `hardware` | `Get-CimInstance Win32_ComputerSystem/_Processor/_PhysicalMemory` | In a QEMU VM several fields are empty or synthetic — exercise 02 must still be passable | |
| `firmware` | `Get-ComputerInfo`, `Get-Disk` | `Get-ComputerInfo` is slow (10–30 s). Does `wb` look hung? | |
| `datentraeger` | `Get-Disk`, `Get-PhysicalDisk` | `MediaType`/`BusType` in a VM often read `Unspecified` | |
| `spiegel` | `Get-StoragePool`, `Get-VirtualDisk` | Before exercise 03 there is no pool — must fail *gracefully*, not scarily | |
| `bitlocker` | `manage-bde -status` | **Never executed anywhere.** Needs admin. Long German output | |
| `schutz` | `Get-MpComputerStatus`, `Get-NetFirewallProfile` | **Never executed anywhere.** Defender may be absent on Server 2022 → does `wb` say something useful? | |
| `ordnerliste` | internal | Covered by CI | |

For each capture file also check: **file encoding** (open in Notepad — are
umlauts intact?) and that no absolute path leaks a username (PRD privacy rule).

- [ ] All nine run, all nine files opened and read
- [ ] Findings for anything that produced an error or unreadable text

---

## Part D — Exercise walkthrough

Work exercises 01 → 08 in order, as a learner, using only the exercise text.
Time each one and compare to the estimate in `START_HIER.md` — the handbook's
4-block plan depends on those numbers being roughly right.

| # | Exercise | Windows-specific surface CI never touched | Time | Pass |
|---|---|---|---|---|
| 01 | Dein Server, deine Firma | — (text and answers only) | | |
| 02 | Was steckt in der Kiste? | `Get-CimInstance` values in a QEMU VM; ADR 0005 says these are presence-checked, verify honest work is not rejected | | |
| 03 | Zwei Platten, ein Spiegel | `diskpart`, storage pool creation, `Get-VirtualDisk` | | |
| 04 | Fingerabdruck & Backup | `Get-FileHash`, `robocopy`, `vssadmin` — **and** the „Falls du Windows 11 nutzt" VSS note | | |
| 05 | Daten weg — und zurück | Restore path; `robocopy` exit codes (robocopy returns 1 on success — does any check trip over that?) | | |
| 06 | Die Platte stirbt | Disk removal/reattach after VM restart; the `S:` note in `START_HIER.md` | | |
| 07 | Tresor zu, Tresor auf | `manage-bde`, `Get-BitLockerVolume`, BitLocker feature install on Server 2022 vs Windows 11 | | |
| 08 | Generalprobe | Full run, plus every cleanup step | | |

For each exercise: does `wb check` give a **hint that actually helps** when you
deliberately get it wrong once? Try one wrong answer per exercise on purpose.

- [ ] 01–08 complete on Windows Server 2022
- [ ] 04 and 07 re-checked on Windows 11 (the two exercises with divergent notes)
- [ ] Every cleanup step leaves the machine as found

---

## Part E — Report and privacy

| # | Step | Expected | Result |
|---|---|---|---|
| E1 | `.\wb bericht` | Asks for a name once, writes `bericht.txt` | |
| E2 | Open `bericht.txt` | Readable German, umlauts intact, integrity hash present | |
| E3 | Read it as if you were the data protection officer | Alias only. No username, no hostname, no serial numbers, no paths that identify a person | |
| E4 | `.\wb bericht` again | Remembers the name, does not ask twice | |
| E5 | Search the whole folder for the answers | Only hashes. No plaintext solution anywhere (CLAUDE.md rule 6) | |

- [ ] E1–E5 done

---

## Part F — Findings

| # | Part | What happened | Severity | Blocks pilot? | Fixed in |
|---|---|---|---|---|---|
| 1 | | | | | |
| 2 | | | | | |
| 3 | | | | | |

Severity: **blocker** (a learner cannot continue alone) · **friction** (works,
but costs the trainer an interruption) · **cosmetic**.

The bar is deliberately not "no findings". The bar is: **a nervous beginner in
front of this VM gets from ZIP to a green exercise 01 without asking a human a
mechanical question.** Anything that breaks that is a blocker.

---

## Exit criteria

- [ ] Parts A–E completed on Windows Server 2022
- [ ] All **blocker** findings fixed, and a new ZIP built from a new tag
- [ ] The re-run of Parts A and B on that new ZIP is clean
- [ ] `START_HIER.md` quotes match what Windows actually says
- [ ] Findings that are not fixed are written into `docs/MILESTONES.md` so they
      are visible on pilot day rather than rediscovered in front of learners

Then, and only then, M3 item 2 (external beta) starts — see
`trainer/BETA_FEEDBACK.md` for the sheet the beta tester fills in.
