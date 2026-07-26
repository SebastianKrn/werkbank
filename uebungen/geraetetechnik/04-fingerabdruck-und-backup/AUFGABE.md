# Übung 04 — Fingerabdruck & Backup

**Ziel:** Du nimmst den Fingerabdruck einer Datei (SHA-256), sicherst die Daten
mit `robocopy` — einmal voll, einmal nur die Änderungen — und legst eine
Schattenkopie an.

ca. 40 Minuten · Schwierigkeit 2 von 3 · prüfungsrelevant (LB): ja

Ein **Hash** ist der Fingerabdruck einer Datei: eine lange Zeichenfolge, die
sich ändert, sobald sich ein einziges Byte ändert. Damit kannst du beweisen,
dass eine Datei unversehrt ist — nach dem Kopieren, nach dem Wiederherstellen,
nach einem Transport.

## Vorher

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\04-fingerabdruck-und-backup\abgabe"
```

Du brauchst die Firmendaten aus Übung 01 (`C:\wb\firma`) und das Spiegel-Laufwerk
`S:` aus Übung 03.

> **Falls dein Spiegel nicht steht:** Nimm überall statt `S:\backup` den Ordner
> `C:\wb\backup`. Die Übung geht genauso — sie prüft die Protokolle, nicht das
> Ziellaufwerk. Hol den Spiegel später mit dem Trainer nach.

## Schritte

### 1. Fingerabdruck nehmen

```powershell
Get-FileHash C:\wb\firma\vertraege\vertrag-001.txt -Algorithm SHA256 |
    Format-List | Out-File "$a\hash-vorher.txt"
notepad "$a\hash-vorher.txt"
```

In der Datei stehen drei Zeilen: `Algorithm`, `Hash` und `Path`. Der Hash ist
64 Zeichen lang.

Trag den Hash in deine Antworten ein (Schritt 5) — du brauchst ihn in Übung 05
wieder, dort ist er dein Beweismittel.

### 2. Vollbackup mit robocopy

```powershell
robocopy C:\wb\firma S:\backup\firma /E /LOG:"$a\backup-voll.log"
notepad "$a\backup-voll.log"
```

- `/E` kopiert alle Unterordner, auch leere.
- `/LOG:` schreibt das Protokoll in eine Datei statt auf den Bildschirm.

Im Protokoll steht unten eine Tabelle: wie viele Verzeichnisse und Dateien
insgesamt, kopiert, übersprungen. Beim ersten Lauf ist alles „kopiert".

**Wichtig:** `robocopy` meldet sich mit einem Rückgabewert. `0` heißt „nichts zu
tun", `1` heißt „erfolgreich kopiert". **Beides ist Erfolg** — anders als bei
fast allen anderen Programmen. Ab `8` ist es ein echter Fehler. Du siehst den
Wert mit:

```powershell
$LASTEXITCODE
```

### 3. Etwas ändern

Eine neue Rechnung kommt herein:

```powershell
"Rechnung 2026-002 - 380 Euro" | Out-File C:\wb\firma\rechnungen\rechnung-002.txt
```

### 4. Zweites Backup — nur die Änderung

Derselbe Befehl, anderes Protokoll:

```powershell
robocopy C:\wb\firma S:\backup\firma /E /LOG:"$a\backup-inkrementell.log"
notepad "$a\backup-inkrementell.log"
```

Vergleiche die beiden Protokolle. Im zweiten steht nur noch **eine** Datei:
`rechnung-002.txt`. Alles andere war schon gesichert und wurde übersprungen.

Genau das ist der Unterschied zwischen einem vollen und einem **inkrementellen**
Backup: Das inkrementelle sichert nur, was sich geändert hat. Es ist schneller
und braucht weniger Platz — aber ohne das Vollbackup davor ist es wertlos.

### 5. Schattenkopie anlegen

Eine **Schattenkopie** (VSS, Volume Shadow Copy) ist ein Standbild eines
Laufwerks — auch von Dateien, die gerade in Benutzung sind. Damit sichert man
Datenbanken und offene Dateien.

Auf **Windows Server**:

```powershell
vssadmin create shadow /for=C:
```

Danach — auf jedem Windows:

```powershell
vssadmin list shadows | Out-File "$a\schattenkopie.txt"
notepad "$a\schattenkopie.txt"
```

In der Datei muss mindestens eine Schattenkopie mit einer Kennung in
geschweiften Klammern stehen, z. B. `{a1b2c3d4-...}`.

### 6. Antworten eintragen

```powershell
Copy-Item "uebungen\04-fingerabdruck-und-backup\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

### 7. Prüfen

```powershell
.\wb check 04
```

## Falls du Windows 11 nutzt

`vssadmin create shadow` gibt es **nur auf Server-Systemen**. Auf Windows 11
legst du die Schattenkopie so an:

```powershell
([WMICLASS]"root\cimv2:Win32_ShadowCopy").Create("C:\", "ClientAccessible")
```

Danach funktioniert `vssadmin list shadows` genau wie beschrieben.

## Aufräumen

Die Schattenkopie belegt Platz und gehört nicht in den Dauerbetrieb einer
Übungsmaschine:

```powershell
vssadmin delete shadows /for=C: /oldest
```

Auf Windows 11 stattdessen:

```powershell
Get-CimInstance Win32_ShadowCopy | Select-Object -First 1 | Remove-CimInstance
```

Und den Arbeitsordner dieser Übung wegräumen:

```powershell
New-Item -ItemType Directory -Path C:\wb\temp-04 -Force
Remove-Item C:\wb\temp-04 -Recurse -Force
Test-Path C:\wb\temp-04 | Out-File "$a\aufraeumen.txt"
```

**Das Backup auf `S:\backup` bleibt stehen.** Du brauchst es in Übung 05 —
dort wirst du Daten löschen und aus genau diesem Backup zurückholen.

## Abgabe

Im Ordner `abgabe`:

- `hash-vorher.txt` — der Fingerabdruck
- `backup-voll.log` — das erste Backup
- `backup-inkrementell.log` — das zweite, mit nur einer Datei
- `schattenkopie.txt` — die Liste der Schattenkopien
- `antworten.toml` — zwei Antworten
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: danach

**Zuerst selbst.** Danach lohnt die Frage: „Welche robocopy-Schalter braucht
man für ein Backup mit Rechten und Zeitstempeln?" Vergleiche die Antwort mit
`robocopy /?` — dort steht die Wahrheit.

## Reflexion

Du hast jetzt ein Backup auf dem Spiegel. Warum ist ein Backup auf demselben
Server trotzdem kein vollständiger Schutz?

## Bonus (freiwillig)

**Ändere ein einziges Zeichen** und beobachte den Fingerabdruck:

```powershell
New-Item -ItemType Directory -Path C:\wb\temp-04 -Force
Copy-Item C:\wb\firma\vertraege\vertrag-001.txt C:\wb\temp-04\kopie.txt
"X" | Add-Content C:\wb\temp-04\kopie.txt
(Get-FileHash C:\wb\temp-04\kopie.txt -Algorithm SHA256).Hash
```

Trag den neuen Hash bei `hash_nach_aenderung` ein und beantworte
`hash_aendert_sich`. Räume danach `C:\wb\temp-04` wieder weg (siehe Aufräumen).

## Homelab (freiwillig, für Erfahrene)

Sichere `C:\wb\firma` auf ein zweites Gerät oder einen USB-Stick — mit
`robocopy`, `rsync` oder deinem eigenen Backup-Werkzeug. Schreib
**mindestens 2 Zeilen** dazu: Was war anders, was hat länger gedauert?

```powershell
notepad "$a\homelab.txt"
```
