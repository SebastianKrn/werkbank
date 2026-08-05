# Werkbank — Gerätetechnik

Willkommen. Du brauchst kein Vorwissen und musst nichts installieren.
Diese Seite reicht für den Anfang.

## In vier Schritten loslegen

**1. Entpacken**

Hast du die ZIP-Datei aus dem Internet geladen? Dann zuerst einmal freigeben:
Rechtsklick auf die ZIP-Datei → „Eigenschaften“ → ganz unten „Zulassen“
ankreuzen → „OK“. Windows markiert alles, was aus dem Internet kommt. Wenn du
das *vor* dem Entpacken machst, ist Ruhe. Machst du es danach, fragt Windows
bei jeder einzelnen Datei nach.

Dann entpacken: Rechtsklick auf die ZIP-Datei → „Alle extrahieren“ → nach `C:\`.
Der Ordner muss auf die Festplatte, nicht in den ZIP-Vorschau-Ordner.
Du hast dann den Ordner `C:\werkbank-geraetetechnik`.

**2. PowerShell im Ordner öffnen**

Rechtsklick auf das Start-Symbol → „Windows PowerShell (Administrator)“ oder
„Terminal (Administrator)“. Dann:

```powershell
cd C:\werkbank-geraetetechnik
```

**3. Schauen, wo du stehst**

```powershell
.\wb status
```

Du siehst alle acht Übungen und welche du schon geschafft hast. Ganz unten steht
dein nächster Schritt.

**4. Die erste Übung öffnen**

```powershell
notepad uebungen\01-dein-server-deine-firma\AUFGABE.md
```

Arbeite die Aufgabe durch. Wenn du glaubst, fertig zu sein:

```powershell
.\wb check 01
```

Fertig. Das ist der ganze Ablauf: **lesen → arbeiten → `wb check`**.

## Die Befehle

| Befehl | Was er macht |
|---|---|
| `.\wb status` | Zeigt alle Übungen, deinen Fortschritt und den nächsten Schritt. |
| `.\wb check` | Prüft die Übung, an der du gerade bist. |
| `.\wb check 03` | Prüft eine bestimmte Übung. |
| `.\wb erfasse <name> 03` | Schreibt eine Systemabfrage in den Abgabe-Ordner der Übung. |
| `.\wb bericht` | Schreibt deinen Fortschritt in `bericht.txt` — das gibst du ab. |
| `.\wb hilfe` | Erklärt alle Befehle auf Deutsch. |

Sieht die Ausgabe komisch aus (Kästchen statt Symbole)? Dann hänge `--ascii` an,
zum Beispiel `.\wb status --ascii`.

## Was du wissen solltest

**Du bekommst Hinweise, keine Lösungen.** Wenn etwas nicht passt, sagt dir `wb`,
*wo* du nachschauen sollst — nicht, was die Antwort ist. Das ist Absicht. Die
Prüfung fragt dich, nicht das Programm.

**Nichts wird verschickt.** Werkbank hat keine Internetverbindung, kein Konto,
keinen Server. Dein Fortschritt liegt in einem Ordner auf deinem Rechner. Den
Bericht gibst du selbst weiter — oder nicht.

**Mach Snapshots.** Du arbeitest in einer virtuellen Maschine. Bevor eine Übung
etwas am System ändert, mach eine Momentaufnahme (Snapshot). Wenn etwas
schiefgeht, springst du zurück — 30 Sekunden, und die Maschine ist wieder heil.
Kaputtmachen ist erlaubt. Deshalb üben wir in einer VM.

**Du darfst stecken bleiben.** Zehn Minuten selbst probieren, dann fragen. Das
ist keine Schwäche, das ist Arbeitsweise.

**Drei Tiefen pro Übung.** Jede Übung hat einen Pflichtteil (**Basis**), einen
Zusatzteil (**Bonus**) und einen Teil für Leute mit eigenem Labor (**Homelab**).
Bestanden ist eine Übung, wenn der Basis-Teil grün ist. Alles darüber ist für
dich, nicht für die Note.

**KI-Stufe.** Jede Übung sagt oben, ob KI erlaubt ist: `ohne` (allein machen),
`danach` (erst selbst, dann vergleichen) oder `frei`. Niemand kontrolliert das.
Es ist deine Prüfung, die kommt.

## Die acht Übungen

| # | Übung | Zeit |
|---|---|---|
| 01 | Dein Server, deine Firma | 30 min |
| 02 | Was steckt in der Kiste? | 30 min |
| 03 | Zwei Platten, ein Spiegel | 45 min |
| 04 | Fingerabdruck & Backup | 40 min |
| 05 | Daten weg — und zurück | 35 min |
| 06 | Die Platte stirbt | 40 min |
| 07 | Tresor zu, Tresor auf | 35 min |
| 08 | Generalprobe | 60 min |

Die Übungen 03 bis 07 bauen aufeinander auf. Mach sie in der Reihenfolge.

## Wenn etwas nicht geht

| Problem | Das hilft |
|---|---|
| `.\wb` wird nicht gefunden | Du bist im falschen Ordner. `cd C:\werkbank-geraetetechnik`, dann `dir` — liegt `wb.exe` da? |
| „Die Datei kann nicht ausgeführt werden“ | Der Ordner liegt noch im ZIP. Richtig entpacken (Schritt 1). |
| Blaues Fenster: „Der Computer wurde durch Windows geschützt“ | Auf „Weitere Informationen“ → „Trotzdem ausführen“. Das ist keine Virenmeldung. Windows kennt den Absender nicht, weil `wb` kein gekauftes Zertifikat hat — für ein Übungsprogramm ist das normal. Die Datei kommt von deinem Trainer. |
| Die Warnung kommt bei jeder Datei wieder | Die ZIP-Datei war beim Entpacken noch gesperrt. Ordner löschen, ZIP-Datei freigeben (Schritt 1), neu entpacken. |
| „Zugriff verweigert“ | PowerShell als Administrator öffnen. |
| `wb` findet keine Übungen | Du bist zu weit weg vom Ordner. Geh in `C:\werkbank-geraetetechnik` zurück. |
| Nach VM-Neustart fehlt Laufwerk `S:` | Virtuelle Platten werden nicht automatisch angesteckt. Steht in Übung 06 unter „Vorher“. |

Bleibt es hängen: Trainer fragen. Mechanik-Probleme sind kein Lernziel.

---

Viel Erfolg. Kleine Schritte, sofortige Antwort, du weißt immer, wo du stehst.
Genau dafür ist das gebaut.
