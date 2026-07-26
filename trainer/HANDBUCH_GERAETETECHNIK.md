# Trainer-Handbuch — Modul Gerätetechnik

Für Trainer und Supplierende. Dieses Handbuch reicht, um das Modul zu
unterrichten, ohne es vorher gebaut zu haben.

**Was hier nicht steht: Lösungen.** Keine erwarteten Antworten, keine Hashes im
Klartext. Die liegen im privaten Repo `werkbank-loesungen`, eine `LOESUNG.md` je
Übung — mit vollständigem Befehlsweg, allen akzeptierten Schreibweisen, den
typischen Fehlern und der didaktischen Absicht. Wer das Modul unterrichtet,
sollte diese Dateien vorher gelesen haben (CLAUDE.md Regel 6, SPEC §4).

---

## 1. Worum es geht

Acht Übungen, die auf die praktische Leistungsbeurteilung vorbereiten. Jede
Übung trainiert dieselben Kompetenzen wie eine LB-Aufgabe, **aber mit anderen
Parametern** — andere Namen, Größen, Laufwerksbuchstaben, andere Formulierung.
Kein LB-Text ist in dieses Material eingeflossen (Prüfungsintegrität, SPEC §4).

| Übung | Kompetenz | LB |
|---|---|---|
| 01 Dein Server, deine Firma | PowerShell-Einstieg, Ordner und Dateien, Get-ChildItem | A1 |
| 02 Was steckt in der Kiste? | CIM-Inventur, UEFI/BIOS, GPT/MBR, VM erkennen | A2 |
| 03 Zwei Platten, ein Spiegel | diskpart-VHDX, Speicherpool, Mirror, NTFS | A3 |
| 04 Fingerabdruck & Backup | SHA-256, robocopy voll/inkrementell, VSS | A4+A5 |
| 05 Daten weg — und zurück | Löschen, Restore, Hash-Beweis | A6 |
| 06 Die Platte stirbt | Ausfall simulieren, Degraded lesen, Repair | A7 |
| 07 Tresor zu, Tresor auf | BitLocker ohne TPM, Defender, Firewall | A8 |
| 08 Generalprobe | alles selbst, Reflexion, Abbau | alle + R |

Die Reihenfolge ist verbindlich: 03 baut den Spiegel, 04 sichert darauf, 05 holt
zurück, 06 zerstört und repariert, 07 verschlüsselt, 08 baut alles ab. Wer 03
überspringt, kann 04 bis 07 nur mit dem Ausweichpfad machen (steht in den
Aufgaben: `C:\wb\backup` statt `S:\backup`).

## 2. Voraussetzungen im Klassenraum

- Eine VM pro Lernendem: **Windows Server 2022** (die LB-Umgebung) oder
  Windows 11 Pro. Lernende brauchen **Adminrechte in der VM**.
- Die Aufgaben nennen an jeder abweichenden Stelle einen Windows-11-Weg
  („Falls du Windows 11 nutzt"). Betroffen: 04 (VSS), 07 (BitLocker-Feature),
  03/06 (nur Oberfläche).
- **Snapshot-Funktion muss verfügbar und erklärt sein.** Sie ist der didaktische
  Reset-Knopf des ganzen Moduls. Vor jeder eingreifenden Übung: Snapshot.
- Kein Internet nötig. Die `vertiefung`-Links sind freiwillig.
- Verteilung: ZIP per USB oder Netzlaufwerk. Nach `C:\` entpacken lassen — nicht
  auf den Desktop, nicht in einen Pfad mit Leerzeichen oder Umlauten.
- `wb.exe` ist nicht signiert. Beim ersten Start warnt SmartScreen:
  „Weitere Informationen" → „Trotzdem ausführen". **Einmal vorher selbst
  durchspielen**, damit die Ansage im Unterricht sitzt.

## 3. Ablaufplan — 4 Einheiten à ca. 100 Minuten

Die 100 Minuten sind mit Absicht die Länge der LB. Die Gruppe gewöhnt sich an
den Takt.

### Einheit 1 — Einstieg (Übungen 01 + 02)

| Zeit | Was |
|---|---|
| 0–15 | Werkbank vorstellen. ZIP entpacken, `.\wb status` **projizieren**. Ablauf zeigen: lesen → arbeiten → `wb check`. Sagen: „Hinweise, keine Lösungen." |
| 15–20 | Snapshot erklären und einmal gemeinsam machen. |
| 20–50 | Übung 01. Fast alle schaffen sie. Wer früh fertig ist: Bonus + Homelab. |
| 50–85 | Übung 02. |
| 85–100 | `.\wb status` gemeinsam ansehen. Basis/Bonus/Homelab erklären. Erste Reflexionsfrage laut besprechen. |

Ziel der Einheit: **jede Person hat einmal ein grünes Häkchen gesehen.** Das ist
wichtiger als der Inhalt von Übung 01.

### Einheit 2 — Speicher (Übungen 03 + 04)

| Zeit | Was |
|---|---|
| 0–10 | Spiegel/RAID 1 an der Tafel: zwei Platten, jede Datei doppelt. Kein Video, eine Skizze. |
| 10–55 | Übung 03. **Die kritische Übung.** Hier bleiben Leute hängen — Zeit einplanen, herumgehen. |
| 55–95 | Übung 04. |
| 95–100 | Frage in die Runde: „Ihr habt ein Backup auf dem Spiegel — reicht das?" (Antwort in Einheit 3.) |

### Einheit 3 — Ernstfall (Übungen 05 + 06)

| Zeit | Was |
|---|---|
| 0–10 | Rückfrage von letztem Mal auflösen: RAID ≠ Backup. |
| 10–45 | Übung 05. **Kernübung des Moduls.** Vor dem Löschen laut ansagen: „Backup zuerst prüfen!" |
| 45–90 | Übung 06. Wenn möglich einen Ausfall **vorne projizieren** — die Gruppe soll sehen, dass `S:` weiterläuft. |
| 90–100 | Nachbesprechung: Wer hat gemerkt, dass nichts passiert ist? |

### Einheit 4 — Schutz und Generalprobe (Übungen 07 + 08)

| Zeit | Was |
|---|---|
| 0–10 | BitLocker ohne TPM erklären: der Chip fehlt, das Kennwort übernimmt seine Rolle. |
| 10–45 | Übung 07. **Vorher ansagen: Kennwort auf Papier.** Auf Server läuft dazwischen ein Neustart (Feature-Installation) — das kostet Zeit, am besten vorbereiten lassen. |
| 45–95 | Übung 08, Teil 1 und 3. |
| 95–100 | `.\wb bericht` erzeugen und einsammeln. |

Teil 2 von Übung 08 (fünf Reflexionsfragen) passt selten in die Einheit.
**Als Hausaufgabe geben** und in der Folgestunde besprechen — die fünf Fragen
sind der beste Prüfungsvorbereitungs-Stoff des ganzen Moduls.

## 4. Was projizieren

Nur vier Dinge. Alles andere machen die Lernenden selbst.

1. `.\wb status` (Einheit 1) — damit alle den Fortschrittsbalken kennen.
2. Ein fehlgeschlagener `.\wb check` (Einheit 1) — damit ein rotes Häkchen
   normal wirkt und nicht wie Versagen.
3. Die Skizze Spiegel/RAID 1 (Einheit 2).
4. Der Ausfall aus Übung 06 (Einheit 3) — der Moment, in dem `S:` trotz fehlender
   Platte weiterläuft.

## 5. Stolperstellen je Übung

Die vollständige Fehlertabelle steht in `werkbank-loesungen/*/LOESUNG.md`. Das
hier ist die Kurzfassung für den laufenden Unterricht.

**Übung 01**
- Notepad hängt `.txt` an → Datei heißt `szenario.txt.txt`. Erkennen mit
  `Get-ChildItem`.
- „5 Zeilen" heißt 5 **nicht-leere** Zeilen. Wer alles in eine Zeile schreibt,
  bleibt rot.
- `$a` gilt nur im offenen Fenster. Neues Fenster → Variable neu setzen. Diese
  Frage kommt garantiert.

**Übung 02**
- `NumberOfCores` vs. `NumberOfLogicalProcessors` — die klassische Verwechslung.
  Kein Fehler im Check (Presence), aber ein gutes Gespräch.
- `MediaType : Unspecified` in der VM. Kein Defekt, steht in der Aufgabe.
- Die Frage „welcher Partitionsstil gehört zu UEFI?" ist **nicht** „was steht bei
  dir?". Wird verwechselt.

**Übung 03 — hier hängen die meisten**
- VHDX kleiner als 4 GB → Speicherpool nimmt sie nicht. Die Aufgabe sagt 5 GB.
- `CanPool : False`: Platte nicht angesteckt (`attach vdisk` vergessen) oder
  schon initialisiert. Rettung: `Clear-Disk -Number N -RemoveData`.
- **Nach jedem VM-Neustart sind die VHDX abgezogen.** Das ist die häufigste
  Frage der ganzen Reihe. Antwort: `Mount-DiskImage` für beide Platten.
- `(Get-StorageSubSystem).FriendlyName` kann mehrere Werte liefern → dann
  gezielt das Windows-Storage-Subsystem nehmen.

**Übung 04**
- robocopy-Exitcode: `0` und `1` sind **Erfolg**. Wer 1 sieht, denkt „Fehler".
- Beide Läufe ins gleiche Log geschrieben → der zweite Check bleibt rot.
- `vssadmin create shadow` gibt es nur auf Server. Windows-11-Weg steht in der
  Aufgabe.

**Übung 05**
- Schritt 0 (Backup prüfen, **bevor** gelöscht wird) wird übersprungen. Laut
  ansagen. Wer ohne Backup löscht: Snapshot zurück, Übung 04 nachholen.
- Hash stimmt nicht: fast immer ein Kopierfehler oder die falsche Datei gehasht.
  Zeichen zählen — es müssen 64 sein.
- `Move-Item` in einen bestehenden Ordner verschachtelt statt zu ersetzen.

**Übung 06**
- Zu schnell erfasst: Windows braucht 15–30 Sekunden, bis der Ausfall sichtbar
  ist.
- „Ich habe alles kaputt gemacht" — nein. Genau das ist der Lernmoment.
- `InService` heißt: Reparatur läuft noch. Warten, dann neu erfassen.

**Übung 07**
- BitLocker-Feature auf Server fehlt → Installation + Neustart. Danach VHDX
  wieder anhängen.
- Kennwort zu kurz (< 8 Zeichen) wird abgelehnt.
- **Kennwort und Wiederherstellungsschlüssel auf Papier, bevor die
  Schlüsseldatei gelöscht wird.** Steht als Warnung in der Aufgabe; trotzdem
  laut sagen.
- Windows 11 Home hat kein BitLocker → Server-VM nutzen.

**Übung 08**
- `-Append` beim zweiten Aufräum-Beweis vergessen → erste Zeile überschrieben.
- Abbau-Reihenfolge: entschlüsseln → VirtualDisk → Pool → VHDX abziehen →
  Ordner löschen. Wer den Ordner zuerst löschen will, bekommt „Datei wird
  verwendet".
- `Start-Transcript` landet ohne `-Path` in den Dokumenten.

## 6. Heterogene Gruppe: die drei Stufen

Jede Übung hat **eine** Aufgabe und drei Tiefen — es gibt bewusst keine
„leichte" und „schwere" Variante:

- **Basis** — Pflicht, LB-relevant. Nur Basis entscheidet über „bestanden".
- **Bonus** — Vertiefung für Schnelle. Blockiert nie.
- **Homelab** — für Leute mit eigener Hardware (in der Pilotgruppe gibt es
  welche). Auch das blockiert nie.

`.\wb status` zeigt das getrennt an (`Basis 5/5 · Bonus 0/1 · Homelab —`).

Was das im Unterricht bedeutet:

- Wer früh fertig ist, wird **nicht** zum Helfen abkommandiert (das demütigt die
  anderen), sondern auf Bonus/Homelab gesetzt.
- Wer langsam ist, muss nichts überspringen. Basis reicht, und Basis ist genau
  das, was die LB verlangt.
- Sag das laut, am besten in Einheit 1: „Bonus ist kein Muss. Wer nur Basis
  macht, ist auf die Prüfung vorbereitet."

## 7. `bericht.txt` lesen

`.\wb bericht` schreibt zwei Dateien in den Werkbank-Ordner: `bericht.txt`
(zum Lesen) und `bericht.json` (zum Weiterverarbeiten).

Darin steht: Alias (selbst gewählt), pro Übung der Status, Versuchszahl,
Zeitstempel, die Zählung je Stufe und ein Integritäts-Hash.

Wie man ihn liest:

- **Viele Versuche bei einer Übung** = jemand hat gekämpft. Nachfragen, nicht
  bewerten. Das ist die wichtigste Information im ganzen Bericht.
- **Basis grün heißt „hat es getan", nicht „hat es verstanden."** Freitext und
  Reflexionsantworten sind Trainerarbeit — `wb` prüft dort nur, dass etwas
  dasteht (ADR 0005).
- Der Integritäts-Hash erkennt **beiläufiges** Nachbearbeiten der
  Fortschrittsdatei, nichts weiter. Er ist kein Prüfungssiegel; so ist er
  dokumentiert und so soll er benutzt werden.
- Keine Telemetrie, keine Übertragung. Der Lernende gibt den Bericht selbst her.
  Er darf auch nein sagen — die Datenminimierung ist Teil des Versprechens an
  eine verletzliche Zielgruppe (PRD §7.6).

## 8. Antworten prüfen

- `antwort`-Checks vergleichen gesalzene SHA-256-Hashes. Der Klartext steht
  **nur** in `werkbank-loesungen`.
- Fällt eine sachlich richtige Antwort durch, weil eine Schreibweise fehlt: das
  ist ein Content-Bug, kein Lernerfehler. Antwort notieren, weitergeben — die
  Schreibweise wird gehasht und ergänzt (`wb intern hash`, siehe `AUTOREN.md`).
  Im Unterricht: Punkt trotzdem geben.
- `wb loesung <ID>` gibt es. Es zeigt keine Lösung, sondern verweist auf dich.
  Wer danach sucht, findet Didaktik statt Schweigen.

## 9. Wenn die Technik streikt

| Symptom | Sofortmaßnahme |
|---|---|
| `wb.exe` startet nicht | Ordner noch im ZIP; richtig entpacken. |
| SmartScreen blockiert | „Weitere Informationen" → „Trotzdem ausführen". |
| „Zugriff verweigert" | PowerShell als Administrator. |
| `wb` findet keine Übungen | Falscher Ordner. `cd C:\werkbank-geraetetechnik`. |
| Laufwerk `S:` verschwunden | VHDX nach Neustart anhängen (`Mount-DiskImage`). |
| Speicherpool im Eimer | Snapshot zurück. Schneller als jede Reparatur. |
| Übung völlig verfahren | `abgabe`-Inhalt löschen, Snapshot zurück, neu. Fortschritt zählt neu, das ist kein Schaden. |

Mechanik-Probleme sind kein Lernziel. Schnell lösen, weitermachen.

## 10. Nach dem Modul

- `bericht.txt` einsammeln (freiwillig).
- Die fünf Reflexionsfragen aus Übung 08 in der Gruppe besprechen — das ist die
  dichteste Prüfungsvorbereitung im Modul.
- Rückmeldung an Sebastian: Wo hat es geklemmt, welche Antwort war richtig und
  wurde rot, welche Übung war zu lang? Das Material wird danach geändert, nicht
  vorher.
