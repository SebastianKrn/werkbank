# Übung 06 — Die Platte stirbt

**Ziel:** Du lässt eine Platte des Spiegels absichtlich ausfallen, holst die
Daten trotzdem heraus und reparierst den Spiegel wieder.

ca. 40 Minuten · Schwierigkeit 3 von 3 · prüfungsrelevant (LB): ja

Ein Spiegel ist nur so gut wie der Techniker, der damit umgehen kann. Heute
stirbt eine Platte — geplant, in deiner VM, ohne Schaden. Du siehst, was Windows
meldet, was noch geht und was du tun musst.

## Vorher

**Mach einen Snapshot.** Diese Übung nimmt dem laufenden Spiegel eine Platte weg.

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\06-die-platte-stirbt\abgabe"
```

Prüfe, dass der Spiegel steht und gesund ist:

```powershell
Get-VirtualDisk | Format-List FriendlyName,HealthStatus,OperationalStatus
```

Steht `HealthStatus : Healthy`? Gut. Wenn nicht — oder wenn `S:` fehlt, weil du
die VM neu gestartet hast — hänge die Platten wieder an:

```powershell
Mount-DiskImage -ImagePath C:\wb\platten\platte1.vhdx
Mount-DiskImage -ImagePath C:\wb\platten\platte2.vhdx
```

> **Merke:** Virtuelle Platten (VHDX) werden beim Neustart **nicht** automatisch
> angesteckt. Nach jedem Neustart der VM musst du sie wieder anhängen. Das ist
> der häufigste Stolperstein in dieser Übungsreihe.

## Schritte

### 1. Die Platte stirbt

```powershell
Dismount-DiskImage -ImagePath C:\wb\platten\platte2.vhdx
```

`Dismount-DiskImage` zieht die Platte virtuell ab — für Windows sieht es aus,
als hätte jemand das Kabel gelöst. Die VHDX-Datei selbst bleibt liegen; das ist
unser Rettungsanker.

Warte etwa 15 Sekunden, damit Windows den Ausfall bemerkt.

### 2. Den Ausfall festhalten

```powershell
Get-StoragePool wb-pool | Format-List FriendlyName,HealthStatus,OperationalStatus |
    Out-File "$a\spiegel-defekt.txt"
Get-VirtualDisk | Format-List FriendlyName,HealthStatus,OperationalStatus,ResiliencySettingName |
    Out-File "$a\spiegel-defekt.txt" -Append
Get-PhysicalDisk | Format-List FriendlyName,HealthStatus,OperationalStatus |
    Out-File "$a\spiegel-defekt.txt" -Append
notepad "$a\spiegel-defekt.txt"
```

Was du jetzt siehst, ist die Sprache, in der Windows „eine Platte fehlt" sagt:

| Meldung | Bedeutung |
|---|---|
| `HealthStatus : Warning` | „Achtung, aber ich lebe." |
| `OperationalStatus : Degraded` oder `Incomplete` | Der Spiegel läuft nur noch einfach — kein Schutz mehr. |
| `OperationalStatus : Lost Communication` (bei der Platte) | Diese Platte antwortet nicht mehr. |

Auf deutschen Systemen stehen dort deutsche Wörter (`Warnung`,
`Beeinträchtigt`). Beides ist richtig.

### 3. Die Daten retten

Das Wichtigste bei einem Plattenausfall ist nicht die Reparatur. Es sind die
Daten. Hol sie zuerst heraus:

```powershell
Copy-Item S:\backup\firma\vertraege\vertrag-001.txt C:\wb\rettung.txt
Get-Content C:\wb\rettung.txt | Out-File "$a\datei-lesbar.txt"
```

Das Laufwerk `S:` funktioniert weiter, obwohl eine Platte fehlt. **Das** ist der
Sinn eines Spiegels: kein Stillstand, keine Panik, Zeit zum Handeln.

### 4. Die Platte kommt zurück

Ein Techniker hätte jetzt eine neue Platte eingebaut. Bei uns kommt dieselbe
wieder:

```powershell
Mount-DiskImage -ImagePath C:\wb\platten\platte2.vhdx
Get-VirtualDisk wb-spiegel | Repair-VirtualDisk
```

`Repair-VirtualDisk` stellt die zweite Kopie wieder her. Das kann ein paar
Minuten dauern. Schau nach, wie es läuft:

```powershell
Get-VirtualDisk | Format-List FriendlyName,HealthStatus,OperationalStatus
```

Solange dort `InService` steht, arbeitet die Reparatur noch. Warte, bis wieder
`Healthy` dasteht.

### 5. Den reparierten Zustand festhalten

Erst wenn `Healthy` dasteht:

```powershell
Get-VirtualDisk | Format-List FriendlyName,HealthStatus,OperationalStatus |
    Out-File "$a\spiegel-repariert.txt"
```

### 6. Antworten eintragen

```powershell
Copy-Item "uebungen\06-die-platte-stirbt\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

### 7. Prüfen

```powershell
.\wb check 06
```

## Falls du Windows 11 nutzt

Alle Befehle sind identisch. Ein Unterschied: Windows 11 zeigt für einen
beeinträchtigten Speicherplatz zusätzlich eine Meldung in den Einstellungen an —
die Befehle sagen dasselbe, nur früher.

## Aufräumen

Die Rettungskopie war für den Notfall. Der Notfall ist vorbei:

```powershell
Remove-Item C:\wb\rettung.txt -Force
Test-Path C:\wb\rettung.txt | Out-File "$a\aufraeumen.txt"
```

**Der Spiegel bleibt stehen** — aber gesund. Einen beeinträchtigten Spiegel
stehen zu lassen, wäre der schlimmste Fehler dieser Übung: Fällt die zweite
Platte aus, sind die Daten weg.

## Abgabe

Im Ordner `abgabe`:

- `spiegel-defekt.txt` — der Ausfall, dokumentiert
- `datei-lesbar.txt` — die Daten waren trotzdem lesbar
- `spiegel-repariert.txt` — wieder `Healthy`
- `antworten.toml` — zwei Antworten
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: danach

**Zuerst selbst.** Danach eine sehr gute Frage an eine KI: „Was passiert bei
Storage Spaces, wenn ich eine defekte Platte ersetze — muss ich sie dem Pool
hinzufügen?" Prüfe die Antwort an deinem eigenen System nach. Bei Speicherplätzen
erzählen KIs oft Halbwahrheiten, weil sich das Verhalten zwischen
Windows-Versionen geändert hat.

## Reflexion

Der Spiegel hat den Ausfall überlebt. Was wäre passiert, wenn während der
Reparatur die zweite Platte ausgefallen wäre?

## Bonus (freiwillig)

Beobachte die Reparatur mit und trage bei `repair_beobachtung` ein, was du
gesehen hast — wie lange sie dauerte, welche Zustände durchlaufen wurden:

```powershell
while ((Get-VirtualDisk wb-spiegel).HealthStatus -ne "Healthy") {
    Get-VirtualDisk wb-spiegel | Format-Table FriendlyName,HealthStatus,OperationalStatus
    Start-Sleep -Seconds 5
}
```

Abbrechen mit `Strg + C`.

## Homelab (freiwillig, für Erfahrene)

Simuliere den gleichen Ausfall in einem anderen System (mdadm, ZFS,
Hardware-RAID). Schreib **mindestens 2 Zeilen** dazu: Wie merkt man dort den
Ausfall, wie wird repariert?

```powershell
notepad "$a\homelab.txt"
```
