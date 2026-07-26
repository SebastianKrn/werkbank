# Übung 05 — Daten weg — und zurück

**Ziel:** Du löschst echte Daten, holst sie aus dem Backup zurück und beweist mit
dem Fingerabdruck, dass sie byte-genau dieselben sind.

ca. 35 Minuten · Schwierigkeit 2 von 3 · prüfungsrelevant (LB): ja

Ein Backup ist erst dann ein Backup, wenn man es einmal zurückgeholt hat. Alles
andere ist Hoffnung. Diese Übung ist die wichtigste der ganzen Reihe.

## Vorher

**Mach einen Snapshot.** Du löschst hier absichtlich Daten.

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\05-daten-weg-und-zurueck\abgabe"
```

Du brauchst das Backup aus Übung 04 (`S:\backup\firma`).

> **Falls dein Spiegel nicht steht:** Nimm statt `S:\backup` deinen Ordner
> `C:\wb\backup`.

Prüfe zuerst, dass das Backup wirklich da ist — **vor** dem Löschen:

```powershell
Get-ChildItem S:\backup\firma -Recurse
```

Wenn hier nichts kommt: **nicht weitermachen.** Erst Übung 04 fertig machen.
Löschen ohne Backup ist kein Übungsschritt, sondern ein Datenverlust.

## Schritte

### 1. Fingerabdruck vor dem Löschen

```powershell
(Get-FileHash C:\wb\firma\vertraege\vertrag-001.txt -Algorithm SHA256).Hash |
    Out-File "$a\hash-vorher.txt"
notepad "$a\hash-vorher.txt"
```

Die runden Klammern bedeuten: „führe das innen zuerst aus, nimm dann davon die
Eigenschaft `.Hash`". So bekommst du nur die 64 Zeichen, ohne Tabelle drumherum.

Trag diesen Wert bei `hash_vorher` in die Antworten ein (Schritt 6).

### 2. Der Schaden

```powershell
Remove-Item C:\wb\firma\vertraege -Recurse -Force
Test-Path C:\wb\firma\vertraege | Out-File "$a\datenverlust.txt"
```

In `datenverlust.txt` steht jetzt `False` — die Verträge sind weg. Im Ernstfall
wäre das der Moment, in dem das Telefon läutet.

### 3. Zurückholen — aber nicht blind

Ein guter Techniker schreibt ein Backup **nicht** direkt über den Schaden. Er
holt es erst daneben zurück und schaut es an. Wenn das Backup selbst kaputt ist,
hat man sonst nichts mehr.

```powershell
robocopy S:\backup\firma\vertraege C:\wb\restore-probe /E /LOG:"$a\restore.log"
notepad "$a\restore.log"
```

`C:\wb\restore-probe` ist dein Prüfstand.

### 4. Fingerabdruck vergleichen

```powershell
(Get-FileHash C:\wb\restore-probe\vertrag-001.txt -Algorithm SHA256).Hash |
    Out-File "$a\hash-nachher.txt"
notepad "$a\hash-nachher.txt"
```

Vergleiche die beiden Dateien `hash-vorher.txt` und `hash-nachher.txt`. Sind die
64 Zeichen gleich? Dann ist die Datei **byte-genau** dieselbe. Nicht „sieht
gleich aus" — dieselbe.

PowerShell kann auch selbst vergleichen:

```powershell
(Get-Content "$a\hash-vorher.txt") -eq (Get-Content "$a\hash-nachher.txt")
```

Trag den Wert bei `hash_nachher` ein. `wb` prüft dann, ob `hash_vorher` und
`hash_nachher` übereinstimmen — das ist dein Beweis.

### 5. Auf den richtigen Platz stellen

Das Backup ist geprüft. Jetzt darf es an seinen Platz:

```powershell
Move-Item C:\wb\restore-probe C:\wb\firma\vertraege
Get-ChildItem C:\wb\firma\vertraege | Out-File "$a\wiederhergestellt.txt"
```

`Move-Item` verschiebt — der Prüfstand ist damit von selbst verschwunden. Das
ist sauberes Arbeiten: kein Schritt zum Aufräumen übrig.

### 6. Antworten eintragen

```powershell
Copy-Item "uebungen\05-daten-weg-und-zurueck\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

### 7. Prüfen

```powershell
.\wb check 05
```

## Falls du Windows 11 nutzt

Diese Übung läuft auf Windows 11 unverändert. `robocopy`, `Get-FileHash`,
`Move-Item` und `Test-Path` sind überall gleich.

## Aufräumen

```powershell
Test-Path C:\wb\restore-probe | Out-File "$a\aufraeumen.txt"
```

Wenn du Schritt 5 gemacht hast, steht dort `False` — der Prüfstand ist weg,
weil du ihn verschoben hast. Steht `True`, liegt noch eine doppelte Kopie
deiner Daten auf der Platte. Genau das findet ein Prüfer.

## Abgabe

Im Ordner `abgabe`:

- `hash-vorher.txt` und `hash-nachher.txt` — die beiden Fingerabdrücke
- `datenverlust.txt` — enthält `False`
- `restore.log` — das Protokoll der Wiederherstellung
- `wiederhergestellt.txt` — die Dateien sind wieder da
- `antworten.toml` — mit `hash_vorher` und `hash_nachher`
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: ohne

**Diese Übung machst du ohne KI.** Sie ist der Kern des ganzen Moduls. Wer den
Restore einmal selbst gemacht hat, kann ihn im Ernstfall. Wer ihn sich erklären
lässt, kann ihn erzählen. Das ist ein Unterschied, den man erst merkt, wenn es
brennt.

## Reflexion

Du hast bewiesen, dass die Datei unversehrt zurückkam. Was hättest du getan,
wenn die beiden Fingerabdrücke **nicht** übereingestimmt hätten?

## Bonus (freiwillig)

Beantworte bei `beweis`, womit du die Unversehrtheit bewiesen hast — das
Fachwort dafür.

## Homelab (freiwillig, für Erfahrene)

Hol dasselbe Backup auf einem **anderen** Gerät zurück (andere VM, anderes
Betriebssystem). Prüfe dort den Hash. Schreib **mindestens 2 Zeilen** dazu:

```powershell
notepad "$a\homelab.txt"
```
