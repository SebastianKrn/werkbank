# Übung 02 — Was steckt in der Kiste?

**Ziel:** Du liest aus, welche Hardware dein Server hat, und schreibst die
wichtigsten Werte auf.

ca. 30 Minuten · Schwierigkeit 1 von 3 · prüfungsrelevant (LB): ja

Ein Techniker, der ein Gerät vor sich hat, fragt immer zuerst: *Was ist das
überhaupt?* Prozessor, Arbeitsspeicher, Firmware, Datenträger. Genau das machst
du jetzt — nicht durch Hingucken, sondern mit Befehlen. Deine VM erzählt dir
alles, wenn du sie richtig fragst.

## Vorher

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\02-was-steckt-in-der-kiste\abgabe"
```

Snapshot brauchst du hier nicht — du veränderst nichts am System, du liest nur.

## Schritte

### 1. Drei Abfragen erfassen

`wb` bringt fertige Abfragen mit. Sie schreiben ihre Ausgabe direkt in deinen
Abgabe-Ordner. Tippe der Reihe nach:

```powershell
.\wb erfasse hardware 02
.\wb erfasse firmware 02
.\wb erfasse datentraeger 02
```

Das `02` sagt `wb`, zu welcher Übung die Datei gehört. Du kannst auch den ganzen
Namen tippen, `02` reicht aber.

Was dahinter steckt (das sind die Befehle, die du auch bei der Prüfung selbst
tippen können musst):

| `wb erfasse …` | fragt ab mit |
|---|---|
| `hardware` | `Get-CimInstance Win32_ComputerSystem`, `Win32_Processor`, `Win32_PhysicalMemory` |
| `firmware` | `Get-ComputerInfo -Property BiosFirmwareType`, `Get-Disk` |
| `datentraeger` | `Get-Disk`, `Get-PhysicalDisk` |

**CIM** ist die Schnittstelle, über die Windows Auskunft über sich selbst gibt.
`Get-CimInstance` holt sich dort eine Klasse, z. B. `Win32_Processor`.

### 2. Lies die Dateien

```powershell
notepad "$a\hardware.txt"
notepad "$a\firmware.txt"
notepad "$a\datentraeger.txt"
```

Suche in `hardware.txt` nach:

- `NumberOfCores` — wie viele **Kerne** hat der Prozessor?
- `TotalPhysicalMemory` — der Arbeitsspeicher in **Byte**. Teile durch
  1073741824, dann hast du Gigabyte. PowerShell rechnet für dich:

  ```powershell
  (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB
  ```

Suche in `firmware.txt` nach:

- `BiosFirmwareType` — steht dort `Uefi` oder `Bios`?
- `PartitionStyle` — steht dort `GPT` oder `MBR`?

Suche in `datentraeger.txt` nach:

- `FriendlyName` und `BusType` — was für ein Datenträger ist das?
- `MediaType` — echte Festplatte oder SSD? In einer VM steht hier oft
  `Unspecified`. Das ist kein Fehler: Die VM sieht keine echte Platte, sondern
  eine virtuelle.

### 3. Trag deine Inventur ein

Vorlage kopieren und ausfüllen:

```powershell
Copy-Item "uebungen\02-was-steckt-in-der-kiste\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

Jede Antwort steht in **einfachen** Anführungszeichen. Beispiel für die Form
(nicht abschreiben, das sind nicht deine Werte):

```toml
kerne = '2'
ram_gb = '4'
firmware_typ = 'Uefi'
partitionsstil = 'GPT'
vm_indiz = '...'
```

Warum einfache Anführungszeichen? In einfachen darfst du alles schreiben — auch
Pfade wie `C:\wb\firma`. In doppelten wäre der Backslash `\` ein Sonderzeichen,
und die ganze Datei wäre kaputt. Das gilt in allen Übungen.

Bei `vm_indiz`: **Woran erkennst du, dass das eine virtuelle Maschine ist und
kein echter Server?** Ein Satz reicht. Schau dir dazu in `hardware.txt` die
Felder `Manufacturer` und `Model` an.

Bei `passt_zu_uefi`: Eine Wissensfrage, die für alle gleich ist — welcher
Partitionsstil gehört technisch zu UEFI, `MBR` oder `GPT`? (Vorsicht: Das ist
**nicht** dieselbe Frage wie „was steht bei dir?“ — es kann sein, dass beides
gleich ist, muss aber nicht.)

### 4. Notiz in den Arbeitsordner legen

Im Alltag legt man sich so eine Inventur in den Arbeitsordner:

```powershell
Copy-Item "$a\hardware.txt" C:\wb\inventur-notiz.txt
```

### 5. Aufräumen

Die Notiz aus Schritt 4 war Arbeitsmaterial und muss weg:

```powershell
Remove-Item C:\wb\inventur-notiz.txt -Force
Test-Path C:\wb\inventur-notiz.txt | Out-File "$a\aufraeumen.txt"
```

In der Datei muss `False` stehen. Deine Abgabe-Dateien bleiben natürlich liegen —
weggeräumt wird nur, was du im System angelegt hast.

### 6. Prüfen

Erst aufräumen, dann prüfen — `wb` schaut auch auf das Aufräumen:

```powershell
.\wb check 02
```

## Abgabe

Im Ordner `abgabe`:

- `hardware.txt`, `firmware.txt`, `datentraeger.txt` — die drei Abfragen
- `antworten.toml` — deine Inventur
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: danach

**Zuerst selbst.** Lies die Dateien und beantworte alles allein. Erst wenn
`wb check 02` grün ist, darfst du eine KI fragen — und zwar am besten so:
„Erklär mir, was `Win32_PhysicalMemory` noch alles liefert.“ Du prüfst dann, ob
die Antwort zu deiner Datei passt. Wer zuerst fragt, hat die Übung nicht
gemacht, sondern zugeschaut.

## Reflexion

Warum fragt man die Hardware mit einem Befehl ab, statt im Geräte-Manager
nachzuschauen?

## Bonus (freiwillig)

Trag bei `medientyp_erklaerung` ein, warum in einer VM bei `MediaType` oft
`Unspecified` steht — und was das für eine Fehlersuche bedeutet („die Platte ist
langsam“ — wie prüft man das in einer VM überhaupt?).

## Homelab (freiwillig, für Erfahrene)

Führe dieselbe Abfrage auf echter Hardware aus (dein Laptop, ein Raspberry Pi
mit `lscpu`, ein anderer Server). Schreib **mindestens 3 Unterschiede** zur VM
auf:

```powershell
notepad "$a\homelab.txt"
```
