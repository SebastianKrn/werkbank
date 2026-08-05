# TESTPROTOKOLL — Handprüfung vor dem Pilot-Freeze

Verbindlich laut SPEC §8. Läuft auf einer echten Windows-VM, bevor irgendein
ZIP in eine Klasse geht.

Die CI beweist, dass der Code kompiliert, die Inhalte gültig sind und das
Archiv richtig zusammengebaut ist. Sie kann **nicht** beweisen, dass
`manage-bde` das ausgibt, was wir glauben, dass eine deutsche Konsole Umlaute
nicht zerlegt, oder dass ein Anfänger an SmartScreen vorbeikommt.

---

## Bevor du anfängst

| | |
|---|---|
| **Wer** | Sebastian |
| **Dauer** | 2–3 Stunden |
| **Maschine** | frische Windows-Server-2022-VM — Aufbau in `docs/VM_WINDOWS_SERVER.md` |
| **Testobjekt** | `werkbank-geraetetechnik-v0.1.0-rc3.zip` |
| **Bezugsquelle** | <https://github.com/SebastianKrn/werkbank/releases/tag/v0.1.0-rc3> |

**So benutzt du diese Datei:** kopier sie **aus dem Repo heraus** nach
`~/pruefung-2026-08-TT.md` und füll die Kopie aus. Die Datei hier bleibt leer —
sie ist das Messgerät, nicht das Messergebnis.

```bash
cp docs/TESTPROTOKOLL.md ~/pruefung-$(date +%Y-%m-%d).md
```

**Ausgefüllte Protokolle nie committen.** Sie enthalten Maschinendetails.

### Zwei Regeln während des Tests

1. **Du darfst dir nicht selbst helfen.** Brauchst du für einen Schritt Wissen,
   das nicht in `START_HIER.md` oder im Aufgabentext steht, ist das ein **Fund**.
   Schreib ihn auf, statt ihn zu umgehen.
2. **Schreib mit, während du arbeitest.** Ärger von vor zwei Stunden fühlt sich
   später harmloser an, als er war.

### Checkliste vor dem Start

- [ ] VM steht, Windows Server 2022 **auf Deutsch**, Desktop-Darstellung
- [ ] IE-Sicherheitskonfiguration ist **Aus** (sonst kein Download möglich)
- [ ] Rücksetz-Kopie `wb-test-FRISCH.qcow2` liegt auf dem Host
- [ ] ZIP **und** `SHA256SUMS.txt` mit Edge **in der VM** heruntergeladen
- [ ] Prüfsumme stimmt (Befehl in `docs/VM_WINDOWS_SERVER.md`, Schritt 6)
- [ ] PowerShell **als Administrator** offen

---

## Warum dieses Protokoll so aufgebaut ist

Zwei Tatsachen bestimmen die Reihenfolge.

**Sieben von neun Erfassen-Presets sind noch nie gelaufen.** Die
Integrationstests führen nur `ordnerliste` und `ipconfig` aus. Die Befehle für
`systeminfo`, `hardware`, `firmware`, `datentraeger`, `spiegel`, `bitlocker`
und `schutz` sind auf keiner Maschine in keiner Pipeline je ausgeführt worden.
`bitlocker` und `schutz` stehen in `runner/src/capture.rs` als `unix: None` und
*können* auf dem Linux-Entwicklungsrechner gar nicht laufen. **Deshalb kommt
Teil C vor Teil D.**

**Die Zeichenkodierung ist nie gegen eine echte Konsole geprüft worden.** Die
Unit-Tests decken UTF-16LE und CP850 über Fixtures ab. Eine deutsche
Windows-Konsole, die PowerShell-Ausgabe in einen Rust-Prozess schiebt, ist
etwas anderes. An Umlauten zeigt es sich.

**Reihenfolge:** A → B → **C** → D → E → F. Teil C zuerst durchsehen; wenn dort
etwas bricht, bricht es beim Piloten auch.

---

## Teil A — Auslieferung und SmartScreen

Schließt den offenen M0-Punkt („SmartScreen-Verhalten einmal auf einer echten
VM prüfen"). `wb.exe` ist unsigniert und bleibt es für den Piloten — kein
Zertifikat, kein Ersatzweg (M0-Entscheidung). Die einzige offene Frage ist, ob
die Anleitung in `START_HIER.md` einen Anfänger durchbringt.

| # | Was du tust | Was passieren soll | Ergebnis |
|---|---|---|---|
| A1 | Rechtsklick auf die ZIP-Datei → **Eigenschaften** | Ganz unten steht ein Kästchen **„Zulassen"** (manchmal „Entsperren") | |
| A2 | Kästchen ankreuzen → **OK**. Dann Rechtsklick → **Alle extrahieren** → Ziel `C:\` | Ordner `C:\werkbank-geraetetechnik` existiert | |
| A3 | `cd C:\werkbank-geraetetechnik` und `.\wb status` | Entweder keine Warnung, oder das blaue Fenster „Der Computer wurde durch Windows geschützt" | |
| A4 | Falls das Fenster kommt: **Weitere Informationen** → **Trotzdem ausführen** | `wb` läuft. **Notier den exakten deutschen Wortlaut des Fensters.** | |
| A5 | **Gegenprobe:** Ordner löschen, ZIP *ohne* „Zulassen" neu entpacken, `.\wb status` | Prüft Zeile 2 der Tabelle „Wenn etwas nicht geht" in `START_HIER.md`. Fragt Windows *nicht* bei jeder Datei nach, ist diese Zeile falsch und muss weg. | |

**Der Wortlaut ist wichtig.** `START_HIER.md` zitiert diesen Dialog. Ein Zitat,
das nicht zum Bildschirm passt, ist schlimmer als gar kein Zitat.

Exakter Wortlaut aus A4:

```
(hier eintragen)
```

- [ ] A1–A5 erledigt

---

## Teil B — Kaltstart

Alles im Ordner `C:\werkbank-geraetetechnik`.

| # | Befehl | Was passieren soll | Ergebnis |
|---|---|---|---|
| B1 | `.\wb status` | Alle acht Übungen, `Fortschritt: 0 von 8`, unten der nächste Schritt | |
| B2 | Umlaute in dieser Ausgabe | `Zwei Platten, ein Spiegel` und `Daten weg — und zurück` stehen korrekt da — Umlaute und Gedankenstrich | |
| B3 | `.\wb status --ascii` | Gleicher Inhalt, keine Kästchen-Symbole | |
| B4 | `.\wb hilfe` | Vollständig deutsch, kein englisches Wort rutscht durch | |
| B5 | `.\wb check 01` (bevor du irgendetwas gemacht hast) | Scheitert freundlich, benennt was fehlt, gibt einen Hinweis und **keine Lösung** | |
| B6 | `.\wb loesung 01` | Didaktische Absage, verweist auf etwas Nützliches | |
| B7 | `type VERSION.txt` | Steht `v0.1.0-rc3` — dasselbe wie auf der Release-Seite | |
| B8 | `notepad uebungen\01-dein-server-deine-firma\AUFGABE.md` | **Das erste, was ein Lernender liest.** Überschrift `Übung 01 — Dein Server, deine Firma`, die Anführungszeichen um „Windows PowerShell (Administrator)“ und jedes `ä ö ü ß` stehen sauber da. Die Datei ist UTF-8 ohne BOM — erkennt Notepad das nicht, sieht der Einstieg zerhackt aus, bevor ein einziger Befehl gelaufen ist | |

Zeigt B2 oder B8 `Ã¼` oder Kästchen: **das ist ein Fund, keine Schönheitsfrage.**
Wer zerhackte deutsche Wörter liest, verliert sofort das Vertrauen ins Werkzeug.

- [ ] B1–B8 erledigt

---

## Teil C — Erfassen-Presets (höchstes Risiko)

**Regel für diesen Teil:** Jedes Preset einmal ausführen **und die erzeugte
Datei öffnen**. Ein Preset, das mit Rückgabewert 0 endet und dabei eine
Fehlermeldung in die Datei schreibt, ist genau der Fehler, den wir hier suchen.
Der Rückgabewert allein sagt nichts.

### C.1 — Die acht, die ohne Vorbereitung laufen

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\01-dein-server-deine-firma\abgabe"

.\wb erfasse systeminfo 01   ; notepad "$a\systeminfo.txt"
.\wb erfasse ipconfig 01     ; notepad "$a\ipconfig.txt"
.\wb erfasse hardware 01     ; notepad "$a\hardware.txt"
.\wb erfasse firmware 01     ; notepad "$a\firmware.txt"
.\wb erfasse datentraeger 01 ; notepad "$a\datentraeger.txt"
.\wb erfasse spiegel 01      ; notepad "$a\spiegel.txt"
.\wb erfasse schutz 01       ; notepad "$a\schutz.txt"
.\wb erfasse ordnerliste 01  ; notepad "$a\ordnerliste.txt"
```

| Preset | Dahinter steckt | Worauf du achtest | Ergebnis |
|---|---|---|---|
| `systeminfo` | `systeminfo` | Codepage der Konsole; deutsche Feldnamen mit Umlauten | |
| `ipconfig` | `ipconfig /all` | Von der CI abgedeckt, sollte langweilig sein | |
| `hardware` | `Get-CimInstance Win32_ComputerSystem` / `_Processor` / `_PhysicalMemory` | In einer QEMU-VM sind mehrere Felder leer oder erfunden. Übung 02 muss trotzdem lösbar bleiben. | |
| `firmware` | `Get-ComputerInfo`, `Get-Disk` | `Get-ComputerInfo` braucht 10–30 Sekunden. **Sieht `wb` in dieser Zeit aus, als hinge es?** | |
| `datentraeger` | `Get-Disk`, `Get-PhysicalDisk` | `MediaType`/`BusType` stehen in einer VM oft auf `Unspecified` | |
| `spiegel` | `Get-StoragePool`, `Get-VirtualDisk` | Vor Übung 03 gibt es keinen Pool. Muss **freundlich** scheitern, nicht bedrohlich. | |
| `schutz` | `Get-MpComputerStatus`, `Get-NetFirewallProfile` | **Nie irgendwo gelaufen.** Defender ist auf Server 2022 ab Werk aktiv — bestätige das oder finde das Gegenteil. | |
| `ordnerliste` | in Rust gebaut | Von der CI abgedeckt | |

### C.2 — `bitlocker`, zweimal

**BitLocker ist auf Windows Server ab Werk nicht installiert** (Microsoft-Doku:
*Install BitLocker on Windows Server*). Damit ist offen, ob `manage-bde.exe`
überhaupt existiert, bevor das Feature installiert ist. Genau das prüfen wir —
denn ein Lernender in Übung 07 steht vor derselben Frage.

**Erst ohne Feature:**

```powershell
Get-WindowsFeature BitLocker      # erwartet: nicht installiert
.\wb erfasse bitlocker 01
notepad "$a\bitlocker.txt"
```

Was steht in der Datei? Trag es wörtlich ein:

```
(hier eintragen)
```

**Dann mit Feature:**

```powershell
Install-WindowsFeature BitLocker -IncludeAllSubFeature -IncludeManagementTools -Restart
```

Die VM startet neu. Danach wieder als Administrator anmelden und:

```powershell
cd C:\werkbank-geraetetechnik
.\wb erfasse bitlocker 01
notepad "uebungen\01-dein-server-deine-firma\abgabe\bitlocker.txt"
```

| | Ergebnis |
|---|---|
| Vor der Installation | |
| Nach der Installation | |
| Sagt `wb` bei Fehlschlag etwas Brauchbares? | |
| Steht der Hinweis „BitLocker erst installieren" früh genug in Übung 07? | |

### C.3 — Für jede der neun Dateien

- [ ] **Umlaute intakt?** In Notepad öffnen und hinschauen.
- [ ] **Kein Benutzername in einem Pfad?** (Datenschutzregel aus dem PRD)
- [ ] **Steht da echter Inhalt oder eine Fehlermeldung?**

- [ ] Alle neun ausgeführt, alle neun Dateien geöffnet und gelesen

---

## Teil D — Die acht Übungen

Arbeite 01 → 08 in dieser Reihenfolge, **als Lernender**, nur mit dem
Aufgabentext. Nicht ins Lösungs-Repo schauen.

**Stoppuhr mitlaufen lassen.** Die Zeiten in `START_HIER.md` und der 4-Block-Plan
im Handbuch sind bis heute geraten. Deine Messung macht sie zu Zahlen.

| # | Übung | Was die CI hier nie berührt hat | Zeit | Bestanden |
|---|---|---|---|---|
| 01 | Dein Server, deine Firma | — (nur Text und Antworten) | | |
| 02 | Was steckt in der Kiste? | `Get-CimInstance`-Werte in einer QEMU-VM. Laut ADR 0005 nur auf Vorhandensein geprüft — **prüf, dass ehrliche Arbeit nicht abgelehnt wird** | | |
| 03 | Zwei Platten, ein Spiegel | `diskpart`, Speicherpool anlegen, `Get-VirtualDisk` | | |
| 04 | Fingerabdruck & Backup | `Get-FileHash`, `robocopy`, `vssadmin` | | |
| 05 | Daten weg — und zurück | Rückweg; `robocopy` gibt bei Erfolg **1** zurück — stolpert irgendein Check darüber? | | |
| 06 | Die Platte stirbt | Platte abhängen/wieder anhängen nach VM-Neustart; der `S:`-Hinweis in `START_HIER.md` | | |
| 07 | Tresor zu, Tresor auf | `manage-bde`, `Get-BitLockerVolume`, BitLocker-Feature (in C.2 schon installiert) | | |
| 08 | Generalprobe | Vollständiger Durchlauf plus jeder Aufräumschritt | | |

**Pro Übung eine bewusste Fehleingabe.** Gib einmal absichtlich etwas Falsches
ein und prüf: **hilft der Hinweis wirklich weiter, oder rätst du nur?**

- [ ] 01–08 auf Windows Server 2022 abgeschlossen
- [ ] Jeder Aufräumschritt lässt die Maschine so zurück, wie er sie vorfand

---

## Teil E — Bericht und Datenschutz

| # | Was du tust | Was passieren soll | Ergebnis |
|---|---|---|---|
| E1 | `.\wb bericht` | Fragt einmal nach einem Namen, schreibt `bericht.txt` | |
| E2 | `notepad bericht.txt` | Lesbares Deutsch, Umlaute intakt, Integritäts-Hash vorhanden | |
| E3 | Lies ihn wie ein Datenschutzbeauftragter | Nur das Alias. Kein Benutzername, kein Rechnername, keine Seriennummer, kein Pfad, der eine Person erkennbar macht | |
| E4 | `.\wb bericht` nochmal | Merkt sich den Namen, fragt nicht zweimal | |
| E5 | Nimm **eine Antwort, die du in Teil D selbst eingetippt hast**, und such im ganzen Ordner danach: `Get-ChildItem -Recurse \| Select-String "<deine Antwort>"` | Treffer nur in deiner eigenen `abgabe\antworten.toml`. In `uebungen\...\exercise.toml` stehen ausschließlich Hashes (CLAUDE.md Regel 6) | |

- [ ] E1–E5 erledigt

---

## Teil F — Sicherheitsnetz: Junction

**Warum das hier steht — und was schon abgedeckt ist.** `wb` weigert sich, aus
dem Übungsordner auszubrechen. Eine *Junction* ist der Ausbruchsweg, den eine
Lernenden-VM tatsächlich herstellen kann: anders als ein NTFS-Symlink braucht
sie weder Administratorrechte noch Entwicklermodus.

| Weg | Stand |
|---|---|
| `wb check` (Prüfungen lesen Dateien) | **Von der CI abgedeckt.** Der Test `junctions_out_of_the_exercise_are_rejected_at_run_time` in `runner/src/checks/tests.rs` läuft seit PR #3 auf windows-latest. |
| `wb erfasse ordnerliste` (rekursives Durchlaufen) | **Offene Lücke.** Dafür gibt es nur einen Unix-Test mit Symlinks (`folder_list_does_not_follow_symlinks_out_of_the_exercise`). Wie sich `DirEntry::metadata()` unter Windows bei einem Reparse-Point verhält, ist hier nie beobachtet worden. |

Teil F prüft beide, aber die Aufmerksamkeit gehört **F1** — dem `ordnerliste`-Weg.

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\01-dein-server-deine-firma\abgabe"

New-Item -ItemType Directory C:\ausserhalb -Force | Out-Null
"streng geheim" | Out-File C:\ausserhalb\geheim.txt

cmd /c mklink /J "$a\ausbruch" "C:\ausserhalb"
```

> **Backslashes benutzen, keine Schrägstriche.** `cmd` liest ein `/` im Pfad
> als Befehlsschalter — `abgabe/ausbruch` schlägt aus diesem Grund fehl. Das ist
> genau der Fehler, der in Commit `e0341dc` im Test steckte.

Jetzt die beiden Wege prüfen, die hinausführen könnten:

```powershell
.\wb erfasse ordnerliste 01
notepad "$a\ordnerliste.txt"

.\wb check 01
```

| Prüfung | Was passieren soll | Ergebnis |
|---|---|---|
| F1 | **Die offene Lücke.** `ordnerliste.txt` enthält **nicht** `geheim.txt`. Notier, wie der Eintrag `ausbruch` dort aussieht — als `[Ordner]` oder als `[Datei ]`. Steht `geheim.txt` drin, ist das ein **Blocker**: `wb` hat Fremdinhalt in die Abgabe geschrieben. | |
| F2 | `wb check 01` stürzt nicht ab und liest nichts aus `C:\ausserhalb` (Bestätigung dessen, was die CI schon prüft) | |
| F3 | Falls `wb` die Junction ablehnt: ist die Meldung verständlich? | |

**Aufräumen** — wichtig, in dieser Form:

```powershell
cmd /c rmdir "$a\ausbruch"
Remove-Item C:\ausserhalb -Recurse -Force
```

> `cmd /c rmdir` entfernt **nur die Verknüpfung**. `Remove-Item` auf eine
> Junction kann in PowerShell 5.1 den Inhalt des Ziels mitlöschen. Deshalb
> zuerst die Junction mit `rmdir` weg, dann erst das echte Verzeichnis.

- [ ] F1–F3 erledigt

---

## Teil G — Funde

| # | Teil | Was passiert ist | Schwere | Blockiert Pilot? | Behoben in |
|---|---|---|---|---|---|
| 1 | | | | | |
| 2 | | | | | |
| 3 | | | | | |
| 4 | | | | | |
| 5 | | | | | |

**Schwere:**

- **Blocker** — ein Lernender kommt allein nicht weiter
- **Reibung** — es geht, kostet aber den Trainer eine Unterbrechung
- **Kosmetik**

Die Messlatte ist absichtlich nicht „keine Funde". Die Messlatte ist:

> **Ein aufgeregter Anfänger kommt vor dieser VM vom ZIP bis zur grünen Übung 01,
> ohne einen Menschen etwas Mechanisches fragen zu müssen.**

Alles, was das bricht, ist ein Blocker.

---

## Abschlussbedingungen

- [ ] Teile A–F auf Windows Server 2022 abgeschlossen
- [ ] Alle **Blocker** behoben und ein neues ZIP aus einem neuen Tag gebaut
      (`docs/RELEASE.md`)
- [ ] Teile A und B gegen dieses neue ZIP wiederholt und sauber
- [ ] Die Zitate in `START_HIER.md` stimmen mit dem überein, was Windows sagt
- [ ] Nicht behobene Funde stehen in `docs/MILESTONES.md` — sichtbar am
      Pilottag, statt vor der Klasse neu entdeckt zu werden

Erst dann startet M3b Punkt 2, die externe Beta.

---

## Was du zurückbringst

Damit die nächste Arbeitssitzung nichts neu herleiten muss:

1. **Teil G**, die Fundtabelle. In die Sitzung einfügen — nicht committen.
2. **Pro Fund:** welcher Teil, was du erwartet hast, was passiert ist, und der
   **exakte deutsche Wortlaut**, wenn ein Dialog oder eine CLI-Meldung beteiligt
   war. Eine Umschreibung reicht nicht, um einen String zu reparieren.
3. **Welche der neun Presets** unbrauchbare Ausgabe erzeugt haben — und was
   stattdessen in der Datei stand.
4. **Deine gemessenen Zeiten** pro Übung aus Teil D.

---

## Anhang — Windows 11, später

**Sechs Übungen** haben einen Abschnitt „Falls du Windows 11 nutzt": 03, 04,
05, 06, 07 und 08. Keiner davon ist geprüft, solange du nur auf Server 2022
testest.

Nicht alle sechs wiegen gleich schwer:

| Übung | Was der Abschnitt sagt | Gewicht |
|---|---|---|
| 04 | `vssadmin create shadow` gibt es nur auf Server-Systemen; auf Windows 11 führt ein **anderer Befehl** zur Schattenkopie | **echter Unterschied** |
| 07 | BitLocker ist auf Windows 11 Pro/Enterprise eingebaut, auf Home gar nicht da | **echter Unterschied** |
| 03, 06, 08 | Befehle identisch, nur die Oberfläche benennt Dinge anders | Wortlaut |
| 05 | läuft unverändert | Zusicherung |

**Das ist bewusst so verschoben.** Die LB läuft auf Windows Server 2022; das
ist der Weg, an dem der Pilot hängt. Die Windows-11-Hinweise betreffen nur
Lernende, die freiwillig auf einer anderen VM arbeiten.

Wenn du sie prüfen willst, brauchst du eine zweite VM (Windows 11 **Pro** — in
Home fehlt BitLocker). Die beiden echten Unterschiede stecken in 04 und 07;
dafür reichen ca. 90 Minuten. Die vier übrigen Abschnitte prüfst du nebenbei
mit, wenn du 03, 05, 06 und 08 dort ohnehin durchspielst. Bis dahin gilt: alle
sechs Abschnitte sind ungeprüft, und das gehört in `docs/MILESTONES.md`.
