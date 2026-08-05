# Übung 03 — Zwei Platten, ein Spiegel

**Ziel:** Du baust aus zwei virtuellen Festplatten einen Spiegel (RAID 1) und
formatierst ihn als Laufwerk `S:`.

ca. 45 Minuten · Schwierigkeit 2 von 3 · prüfungsrelevant (LB): ja

Ein Spiegel schreibt jede Datei doppelt — auf beide Platten. Fällt eine aus,
läuft die Firma weiter. Genau das baust du jetzt selbst, ohne echte Hardware:
Zwei virtuelle Platten (VHDX-Dateien) verhalten sich für Windows wie zwei
angesteckte Platten.

## Vorher

**Mach jetzt einen Snapshot.** Diese Übung greift in die Datenträgerverwaltung
ein. Wenn etwas hängt, ist der Snapshot schneller als jede Reparatur.

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\03-zwei-platten-ein-spiegel\abgabe"
New-Item -ItemType Directory -Path C:\wb\platten -Force
```

PowerShell muss **als Administrator** laufen, sonst darfst du keine Platten
anlegen.

## Schritte

### 1. Zwei virtuelle Platten anlegen

`diskpart` ist das klassische Werkzeug für Datenträger. Es hat seine eigene
Eingabeaufforderung. Starte es:

```powershell
diskpart
```

Die Zeile beginnt jetzt mit `DISKPART>`. Tippe der Reihe nach (jede Zeile mit
Enter bestätigen):

```
create vdisk file="C:\wb\platten\platte1.vhdx" maximum=5120 type=expandable
select vdisk file="C:\wb\platten\platte1.vhdx"
attach vdisk
create vdisk file="C:\wb\platten\platte2.vhdx" maximum=5120 type=expandable
select vdisk file="C:\wb\platten\platte2.vhdx"
attach vdisk
exit
```

Was das heißt:

- `create vdisk` legt die VHDX-Datei an. `maximum=5120` sind 5120 MB = 5 GB.
- `type=expandable` heißt: die Datei wächst mit — sie belegt am Anfang fast
  nichts. (Das Gegenteil wäre `type=fixed`: sofort volle Größe.)
- `attach vdisk` steckt die Platte virtuell an. Erst danach sieht Windows sie.
- `exit` verlässt diskpart.

**Kleiner als 4 GB darf eine Platte nicht sein**, sonst nimmt sie der
Speicherpool später nicht an. Darum 5 GB.

### 2. Nachsehen, ob Windows die Platten sieht

```powershell
.\wb erfasse datentraeger 03
notepad "$a\datentraeger.txt"
```

Du suchst zwei Einträge mit `CanPool : True` — „darf in einen Speicherpool“.
Wenn dort `False` steht, ist die Platte schon in Benutzung oder noch nicht
angesteckt.

### 3. Speicherpool bauen

Ein **Speicherpool** ist ein Topf, in den man Platten legt. Aus dem Topf
schneidet man dann Laufwerke.

```powershell
$platten = Get-PhysicalDisk -CanPool $true
New-StoragePool -FriendlyName wb-pool -StorageSubSystemFriendlyName (Get-StorageSubSystem).FriendlyName -PhysicalDisks $platten
```

`$platten` ist wieder eine Variable — sie hält die beiden gefundenen Platten
fest.

### 4. Den Spiegel anlegen

```powershell
New-VirtualDisk -StoragePoolFriendlyName wb-pool -FriendlyName wb-spiegel -ResiliencySettingName Mirror -UseMaximumSize
```

`-ResiliencySettingName Mirror` ist das entscheidende Stück: **Mirror** =
Spiegel = RAID 1. Alternativen wären `Simple` (kein Schutz) und `Parity`
(Prüfsummen-Verfahren, braucht mindestens drei Platten).

### 5. Formatieren und Laufwerksbuchstaben geben

```powershell
Get-VirtualDisk wb-spiegel | Get-Disk | Initialize-Disk -PartitionStyle GPT -PassThru |
    New-Partition -DriveLetter S -UseMaximumSize |
    Format-Volume -FileSystem NTFS -NewFileSystemLabel SPIEGEL -Confirm:$false
```

Das ist eine **Kette**: initialisieren → Partition anlegen → formatieren. Jeder
Teil gibt sein Ergebnis mit `|` an den nächsten weiter. `-PassThru` heißt: gib
das Ergebnis weiter, statt es zu schlucken.

Danach hast du ein Laufwerk `S:`.

### 6. Ausprobieren

```powershell
"Spiegeltest" | Out-File S:\test.txt
Get-Content S:\test.txt
```

### 7. Alles abgeben

```powershell
.\wb erfasse spiegel 03
Get-Volume -DriveLetter S | Format-List | Out-File "$a\volume.txt"
```

Schau dir `spiegel.txt` an. Wichtige Zeilen:

- `ResiliencySettingName : Mirror`
- `NumberOfDataCopies : 2` — so oft liegt jede Datei da
- `HealthStatus : Healthy` (auf deutschen Systemen `Fehlerfrei`)

### 8. Antworten eintragen

```powershell
Copy-Item "uebungen\03-zwei-platten-ein-spiegel\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

### 9. Aufräumen

Die Testdatei war Arbeitsmaterial:

```powershell
Remove-Item S:\test.txt -Force
Test-Path S:\test.txt | Out-File "$a\aufraeumen.txt"
```

**Pool, Spiegel und Laufwerk S: bleiben stehen.** Du brauchst sie in den
Übungen 04, 05 und 06. Abgebaut wird alles in Übung 08 — mit Beweis.

### 10. Prüfen

Erst aufräumen, dann prüfen — `wb` schaut auch auf das Aufräumen:

```powershell
.\wb check 03
```

## Falls du Windows 11 nutzt

Die Befehle sind **identisch** — Speicherpools gibt es auf Windows 11 genauso.
Zwei Unterschiede:

- In der Oberfläche heißt es „Speicherplätze“ (Systemsteuerung), nicht
  „Speicherpools“ wie im Server-Manager. Wir arbeiten hier ohnehin mit Befehlen.
- Wenn `New-VirtualDisk` über die Größe klagt, ergänze `-ProvisioningType Thin`.

## Abgabe

Im Ordner `abgabe`:

- `datentraeger.txt` — die Platten vor dem Pool
- `spiegel.txt` — Pool und Spiegel
- `volume.txt` — das formatierte Laufwerk
- `antworten.toml` — drei Antworten
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: danach

**Zuerst selbst, mit dieser Anleitung.** Danach ist eine gute KI-Frage:
„Was ist der Unterschied zwischen Storage Spaces und einem Hardware-RAID?“
Diese Frage stellt man erst sinnvoll, wenn man einen Spiegel einmal gebaut hat.

## Reflexion

Ein Spiegel schützt vor einem Plattenausfall. Vor welchen Datenverlusten
schützt er **nicht**?

## Bonus (freiwillig)

Beantworte bei `parity_mindestplatten`, wie viele Platten das Parity-Verfahren
mindestens braucht. Du findest es heraus mit:

```powershell
Get-StoragePool wb-pool | Get-ResiliencySetting | Format-List Name,NumberOfDataCopies,NumberOfColumns,NumberOfDisksPerEnclosure
```

## Homelab (freiwillig, für Erfahrene)

Baue den gleichen Spiegel mit anderer Technik — `mdadm` unter Linux, ZFS-Mirror,
ein Hardware-RAID-Controller. Schreib **mindestens 3 Unterschiede** zu Storage
Spaces auf:

```powershell
notepad "$a\homelab.txt"
```
