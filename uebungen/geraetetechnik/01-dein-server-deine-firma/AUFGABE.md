# Übung 01 — Dein Server, deine Firma

**Ziel:** Du erfindest eine kleine Firma und baust ihre Ordner und Dateien mit PowerShell auf.

ca. 30 Minuten · Schwierigkeit 1 von 3 · prüfungsrelevant (LB): ja

Das ist deine erste Übung. Sie ist absichtlich leicht. Du sollst hier nur zwei
Dinge lernen: wie du in PowerShell einen Ordner und eine Datei anlegst — und wie
`wb check` dir sagt, ob es gepasst hat.

## Vorher

1. Mach einen **Snapshot** von deiner VM (eine Momentaufnahme). Wenn etwas
   schiefgeht, springst du einfach dorthin zurück. Das ist dein Reset-Knopf.

2. Öffne **PowerShell als Administrator**: Rechtsklick auf das Start-Symbol →
   „Windows PowerShell (Administrator)" oder „Terminal (Administrator)".

3. Wechsle in den Werkbank-Ordner — dort, wo `wb.exe` liegt:

   ```powershell
   cd C:\werkbank-geraetetechnik
   ```

   (Wenn dein Ordner anders heißt, nimm deinen Pfad.)

4. Leg dir eine **Variable** für den Abgabe-Ordner an. Das spart dir viel
   Tipparbeit:

   ```powershell
   $a = "uebungen\01-dein-server-deine-firma\abgabe"
   ```

   Eine Variable beginnt in PowerShell immer mit `$`. Sie gilt nur, solange
   dieses Fenster offen ist. Neues Fenster → Zeile noch einmal tippen.

## Schritte

### 1. Denk dir eine Firma aus

Eine kleine Firma, die du dir gut vorstellen kannst. Zum Beispiel eine
Tischlerei, ein Frisörsalon, eine Ordination, ein Vereinsheim.

Du brauchst:

- einen Namen,
- eine Branche,
- einen Ort,
- drei Arten von Daten, die diese Firma speichert (z. B. Verträge, Rechnungen, Fotos),
- einen Namen für den Server (z. B. `aquinas`, `fileserver01`, `srv-berger`).

### 2. Schreib dein Szenario auf

```powershell
notepad "$a\szenario.txt"
```

Notepad fragt, ob die Datei erstellt werden soll → **Ja**.

Schreib **mindestens 5 Zeilen**, eine Angabe pro Zeile. Zum Beispiel so
(nimm deine eigene Firma, nicht diese):

```
Firma: Tischlerei Berger
Branche: Handwerk
Ort: Wien-Simmering
Server: aquinas
Daten: Vertraege, Rechnungen, Fotos
```

Speichern mit `Strg + S`, dann Notepad schließen.

### 3. Leg den Arbeitsordner an

Alles, was du in dieser Übungsreihe außerhalb der Abgabe baust, kommt nach
`C:\wb`. So ist das Aufräumen später leicht.

```powershell
New-Item -ItemType Directory -Path C:\wb -Force
```

`New-Item` ist ein **Cmdlet** (so heißen die Befehle in PowerShell).
`-ItemType Directory` heißt: bitte ein Ordner. `-Force` heißt: kein Fehler, wenn
es den Ordner schon gibt.

### 4. Baue die Ordner deiner Firma

Drei Ordner in einem Befehl — die Namen trennst du mit Komma:

```powershell
New-Item -ItemType Directory -Path C:\wb\firma\vertraege, C:\wb\firma\rechnungen, C:\wb\firma\fotos -Force
```

Und einen Ordner, den du später wieder wegräumst:

```powershell
New-Item -ItemType Directory -Path C:\wb\firma\temp -Force
```

### 5. Leg drei Dateien an

```powershell
"Vertrag 001 - Kueche Familie Huber" | Out-File C:\wb\firma\vertraege\vertrag-001.txt
"Rechnung 2026-001 - 1.240 Euro"     | Out-File C:\wb\firma\rechnungen\rechnung-001.txt
"Foto: Werkstatt, aufgenommen 2026"  | Out-File C:\wb\firma\fotos\werkstatt.txt
```

Der senkrechte Strich `|` heißt **Pipe**. Er schiebt das Ergebnis von links
nach rechts weiter. `Out-File` schreibt es in eine Datei.

### 6. Zeig deine Struktur und speichere sie ab

```powershell
Get-ChildItem C:\wb\firma -Recurse | Out-File "$a\firma-struktur.txt"
```

`Get-ChildItem` listet den Inhalt eines Ordners. `-Recurse` heißt: auch alle
Unterordner. Schau dir die Datei danach an:

```powershell
notepad "$a\firma-struktur.txt"
```

### 7. Prüfen

```powershell
.\wb check 01
```

`wb` sagt dir für jeden Punkt, ob er passt. Wenn etwas offen ist, bekommst du
einen **Hinweis** — nicht die Lösung. Das ist Absicht.

## Aufräumen

Aufräumen ist Teil der Arbeit, nicht das Ende der Arbeit. Bei der
Leistungsbeurteilung kostet vergessenes Aufräumen Punkte. Wir üben es ab jetzt
in jeder Übung.

```powershell
Remove-Item C:\wb\firma\temp -Recurse -Force
Test-Path C:\wb\firma\temp | Out-File "$a\aufraeumen.txt"
```

`Test-Path` antwortet mit `True` (ist da) oder `False` (ist weg). In der Datei
muss `False` stehen — das ist dein Beweis, dass du aufgeräumt hast.

**Der Ordner `C:\wb\firma` bleibt stehen.** Du brauchst ihn in den Übungen 03
bis 06 wieder. Weggeräumt wird er in Übung 08.

## Abgabe

Im Ordner `abgabe`:

- `szenario.txt` — deine Firma, mindestens 5 Zeilen
- `firma-struktur.txt` — die Liste deiner Ordner und Dateien
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: ohne

**Diese Übung machst du ohne KI.** Auch ohne Suchmaschine. Alles, was du
brauchst, steht hier. Fünf Befehle selbst zu tippen ist mehr wert als
fünfzig gelesene.

## Reflexion

Warum ist eine feste Ordnerstruktur in einer Firma wichtiger als auf deinem
privaten Rechner?

## Bonus (freiwillig)

Trag in `abgabe\antworten.toml` ein, mit welchem Cmdlet du die Ordner angelegt
hast. Vorlage kopieren, dann ausfüllen:

```powershell
Copy-Item "uebungen\01-dein-server-deine-firma\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

## Homelab (freiwillig, für Erfahrene)

Baue die gleiche Struktur auf einem zweiten Gerät (zweite VM, Raspberry Pi,
eigener Server). Schreib **mindestens 3 Unterschiede** auf, die dir dabei
aufgefallen sind — z. B. bei Pfaden, Rechten oder Groß-/Kleinschreibung:

```powershell
notepad "$a\homelab.txt"
```
