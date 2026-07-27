# Werkbank in vier Schritten

*(Eine Seite. Zum Ausdrucken und Austeilen — A4, große Schrift. Trainer: bitte in
mindestens 12 pt drucken, Zielgruppe sieht nicht immer gut.)*

---

## 1 · Freigeben, dann entpacken

**Zuerst:** Rechtsklick auf die ZIP-Datei → **Eigenschaften** → ganz unten
**„Zulassen"** ankreuzen → **OK**.

*Vor* dem Entpacken ist Ruhe. Danach fragt Windows bei **jeder** Datei nach.

**Dann:** ZIP-Datei → Rechtsklick → **Alle extrahieren** → nach `C:\`

Du hast dann: `C:\werkbank-geraetetechnik`

---

## 2 · PowerShell öffnen

Rechtsklick auf **Start** → **Windows PowerShell (Administrator)**

Dann eintippen:

```
cd C:\werkbank-geraetetechnik
```

---

## 3 · Wo stehe ich?

```
.\wb status
```

Ganz unten steht dein **nächster Schritt**.

---

## 4 · Arbeiten und prüfen

Aufgabe öffnen:

```
notepad uebungen\01-dein-server-deine-firma\AUFGABE.md
```

Arbeiten. Dann prüfen:

```
.\wb check 01
```

---

# Das war alles

**lesen → arbeiten → `.\wb check`**

---

## Merksätze

**Du bekommst Hinweise, keine Lösungen.** Das ist Absicht. Die Prüfung fragt
dich, nicht das Programm.

**Mach Snapshots.** Vor jeder Übung, die etwas verändert. Kaputtmachen ist
erlaubt — dafür ist die VM da.

**Basis reicht.** Bonus und Homelab sind freiwillig. Wer Basis grün hat, ist auf
die Prüfung vorbereitet.

**Nichts wird verschickt.** Kein Internet, kein Konto, kein Server. Dein
Fortschritt bleibt auf deinem Rechner.

**Zehn Minuten selbst probieren, dann fragen.** Beides ist richtig.

---

## Wenn nichts geht

| Problem | Das hilft |
|---|---|
| `.\wb` nicht gefunden | Falscher Ordner → `cd C:\werkbank-geraetetechnik` |
| Datei lässt sich nicht ausführen | Ordner noch im ZIP → richtig entpacken |
| Windows warnt vor der Datei | „Weitere Informationen" → „Trotzdem ausführen" |
| Zugriff verweigert | PowerShell **als Administrator** öffnen |
| Symbole sehen kaputt aus | `.\wb status --ascii` |

---

## Hilfe auf Deutsch

```
.\wb hilfe
```

---

*Am Ende des Moduls:* `.\wb bericht` — schreibt `bericht.txt`. Diese Datei
gibst du ab.
