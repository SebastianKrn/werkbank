# Übung 08 — Generalprobe

**Ziel:** Du machst alles noch einmal — allein, mit deinen eigenen Werten, von
Anfang bis Ende. Danach baust du den ganzen Aufbau ab und beantwortest fünf
Fragen.

ca. 60 Minuten · Schwierigkeit 3 von 3 · prüfungsrelevant (LB): ja

Bis hierher hast du jeden Schritt mit Anleitung gemacht. Jetzt nicht mehr.
Diese Übung sagt dir **was** zu tun ist, nicht **wie**. Das ist Absicht: Genau
so wirst du bei der Leistungsbeurteilung dasitzen. Wenn du hier hängst, schau in
den Übungen 01 bis 07 nach — die Befehle stehen alle dort. Das ist kein
Schummeln, das ist Nachschlagen. Techniker schlagen nach.

## Vorher

**Mach einen Snapshot.**

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\08-generalprobe\abgabe"
```

Wenn `S:` nach einem Neustart fehlt: Platten anhängen, Laufwerk aufschließen
(es ist seit Übung 07 verschlüsselt).

## Teil 1 — Dein eigenes Szenario (ca. 30 Minuten)

Andere Firma, andere Zahlen, andere Namen als in Übung 01. Du entscheidest.

Arbeite dich durch diese Liste und **schreibe jeden Schritt mit** in
`abgabe\protokoll.txt` — mindestens 12 Zeilen: was du getippt hast, was
herauskam.

1. **Ordner und Daten anlegen.** Ein Ordner `C:\wb\probe` mit mindestens zwei
   Unterordnern und drei Dateien. Inhalt frei erfunden.
2. **Inventur.** Ein Befehl, der zeigt, was für eine Maschine das ist. Ergebnis
   in eine Datei im Abgabe-Ordner.
3. **Fingerabdruck.** Nimm den SHA-256 einer deiner Dateien und trage ihn bei
   `probe_hash_vorher` ein.
4. **Backup.** Sichere `C:\wb\probe` — wohin, entscheidest du (`S:` oder ein
   anderer Ordner). Protokoll mitschreiben lassen.
5. **Schaden.** Lösche einen Unterordner deiner Probe.
6. **Wiederherstellung.** Hol ihn aus dem Backup zurück — erst daneben, dann an
   den Platz.
7. **Beweis.** Nimm den Fingerabdruck derselben Datei erneut und trage ihn bei
   `probe_hash_nachher` ein. Beide Werte müssen gleich sein.

Tipp fürs Protokoll: Du kannst mitschreiben lassen, statt zu tippen.

```powershell
Start-Transcript -Path "$a\protokoll.txt"
# ... arbeiten ...
Stop-Transcript
```

`Start-Transcript` schreibt alles mit, was du tippst und was der Computer
antwortet. Das ist auch im Beruf nützlich: ein Protokoll, das man später
vorlegen kann.

## Teil 2 — Fünf Fragen (ca. 15 Minuten)

Kopiere die Vorlage und beantworte alle fünf Fragen. Jeweils zwei bis vier
Sätze, eigene Worte, keine Stichwortliste.

```powershell
Copy-Item "uebungen\08-generalprobe\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

1. **`reflexion_1`** — Warum ersetzt ein Spiegel (RAID 1) kein Backup?
2. **`reflexion_2`** — Was beweist ein SHA-256-Hash, und was beweist er *nicht*?
3. **`reflexion_3`** — Was ist der Unterschied zwischen einem vollen und einem
   inkrementellen Backup, und warum ist das inkrementelle allein wertlos?
4. **`reflexion_4`** — Wo bewahrst du einen BitLocker-Wiederherstellungsschlüssel
   auf, und warum nicht auf demselben Server?
5. **`reflexion_5`** — Du kommst zu einem fremden Server, den du nicht kennst.
   Welche drei Befehle tippst du zuerst, und was willst du damit wissen?

## Teil 3 — Abbauen (ca. 15 Minuten)

Jetzt kommt der Schritt, den bei Prüfungen die meisten vergessen — und der
Punkte kostet. Alles, was du in sieben Übungen gebaut hast, muss weg.

In dieser Reihenfolge:

```powershell
# 1. Verschlüsselung abschalten (kann ein paar Minuten dauern)
Disable-BitLocker -MountPoint S:
Get-BitLockerVolume -MountPoint S: | Format-List VolumeStatus,EncryptionPercentage

# 2. Warten, bis VolumeStatus FullyDecrypted ist. Dann:
Remove-VirtualDisk -FriendlyName wb-spiegel -Confirm:$false
Remove-StoragePool -FriendlyName wb-pool -Confirm:$false

# 3. Virtuelle Platten abziehen
Dismount-DiskImage -ImagePath C:\wb\platten\platte1.vhdx
Dismount-DiskImage -ImagePath C:\wb\platten\platte2.vhdx

# 4. Arbeitsordner löschen — alles darin
Remove-Item C:\wb -Recurse -Force
```

Und der Beweis, dass wirklich nichts übrig ist:

```powershell
Test-Path C:\wb | Out-File "$a\aufraeumen.txt"
(Get-StoragePool -FriendlyName wb-pool -ErrorAction SilentlyContinue | Measure-Object).Count |
    Out-File "$a\aufraeumen.txt" -Append
notepad "$a\aufraeumen.txt"
```

In der Datei müssen zwei Zeilen stehen: `False` und `0`. Kein Ordner, kein Pool.

Zum Schluss ein Blick auf deine Abgabe:

```powershell
.\wb erfasse ordnerliste 08
```

## Abschluss

```powershell
.\wb check 08
.\wb status
.\wb bericht
```

`wb bericht` schreibt `bericht.txt` und `bericht.json` in den Werkbank-Ordner.
**Diese Datei gibst du ab** — sie zeigt, was du geschafft hast. Sie enthält
keine Antworten und keine persönlichen Daten außer dem Namen, den du selbst
eingibst. Du entscheidest, wem du sie gibst.

## Falls du Windows 11 nutzt

Der Abbau ist identisch. Nur beim Abschalten der Verschlüsselung heißt es auf
Windows 11 in der Oberfläche „BitLocker deaktivieren“ — der Befehl
`Disable-BitLocker` ist derselbe.

## Abgabe

Im Ordner `abgabe`:

- `protokoll.txt` — mindestens 12 Zeilen, dein Weg durch Teil 1
- `antworten.toml` — die beiden Fingerabdrücke, die fünf Reflexionsantworten
  und `wichtigster_schritt`
- `aufraeumen.txt` — enthält `False` und `0`
- `ordnerliste.txt` — der Abschluss-Blick
- alles, was du in Teil 1 sonst noch erfasst hast

## KI-Stufe: ohne

**Diese Übung machst du ohne KI.** Sie ist die Generalprobe. Bei der
Leistungsbeurteilung sitzt auch keine KI neben dir. Nachschlagen in den Übungen
01 bis 07 ist erlaubt und ausdrücklich erwünscht — ein Techniker, der weiß, *wo*
etwas steht, ist ein guter Techniker.

Wenn du nach dieser Übung eine KI fragen willst: Lass dir deine fünf Antworten
kritisieren. Nicht verbessern lassen — kritisieren. Und dann entscheide selbst,
was davon stimmt.

## Reflexion

Welchen Schritt vergessen bei Prüfungen die meisten? **Ein Wort** — trag es bei
`wichtigster_schritt` ein. Du hast ihn in jeder der acht Übungen gemacht.

Und eine Frage nur für dich, ohne Eintrag und ohne Prüfung: Was war in diesen
acht Übungen der Schritt, bei dem du am meisten gelernt hast?

## Bonus (freiwillig)

Gib deinen Probe-Ordner als Netzwerkfreigabe frei und trage bei `smb_freigabe`
ein, welchen Befehl du benutzt hast und wer darauf zugreifen darf:

```powershell
Get-Command -Module SmbShare
```

## Homelab (freiwillig, für Erfahrene)

Mach die ganze Generalprobe auf einem anderen System — Linux mit `mdadm`,
`rsync`, `sha256sum`, `cryptsetup`. Schreib **mindestens 3 Zeilen**: Welche
Schritte waren dort einfacher, welche schwieriger?

```powershell
notepad "$a\homelab.txt"
```
