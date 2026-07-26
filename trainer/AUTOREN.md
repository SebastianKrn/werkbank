# Übungen schreiben — Handbuch für Autoren

Eine neue Übung entsteht durch **einen Ordner mit einer `exercise.toml`**. Der
Runner wird dabei nicht angefasst. Wenn eine Übungsidee eine Runner-Änderung
zu brauchen scheint: stehen bleiben und fragen (SPEC §9).

## 1. Ordner anlegen

```
uebungen/<modul>/<id>/
├── AUFGABE.md                       Aufgabentext (Deutsch, ~B1)
├── exercise.toml                    Metadaten + Checks
├── abgabe/.gitkeep                  hier schreibt der Lernende
└── material/                        optional: Vorlagen, Beispieldaten
    └── antworten-vorlage.toml
```

`<id>` **muss** dem Ordnernamen entsprechen und darf nur `a-z`, `0-9`, `-`, `_`,
`.` enthalten. Konvention: `NN-kurzer-titel`.

## 2. `exercise.toml`

```toml
[exercise]
id = "09-beispiel"          # = Ordnername
titel = "Ein Beispiel"      # erscheint in wb status
modul = "geraetetechnik"
schwierigkeit = 2           # 1-3
zeit_minuten = 30           # realistisch, nicht optimistisch
ki_stufe = "danach"         # ohne | danach | frei
lb_relevant = true          # zählt in den LB-Zähler von wb status
vertiefung = [              # optional, http(s), nie Voraussetzung
  "https://learn.microsoft.com/…",
]

[[check]]
id = "etwas-vorhanden"
type = "file_exists"
path = "abgabe/datei.txt"
stufe = "basis"             # optional: basis (Standard) | bonus | homelab
hint_de = "Ein Hinweis, keine Lösung."
```

Jede Übung braucht **mindestens einen Basis-Check**, sonst wäre sie ohne Arbeit
bestanden — `wb intern lint` weist das ab.

## 3. Die sechs Check-Typen — und keinen siebten

| type | Prüft | Felder |
|---|---|---|
| `file_exists` | Datei existiert und ist nicht leer | `path` |
| `file_matches` | Regex trifft auf den Dateiinhalt | `path`, `pattern` |
| `antwort` | Antwort in `antworten.toml`, gesalzener SHA-256 | `key`, `salt`, `expect_hash`, optional `file` |
| `datei_zeilen_min` | mindestens N nicht-leere Zeilen | `path`, `min` |
| `alle_antworten` | alle genannten Keys sind vorhanden und gefüllt | `keys`, optional `file` |
| `werte_gleich` | zwei Antworten sind nach Normalisierung gleich | `key_a`, `key_b`, optional `file` |

Pfade sind **immer relativ zum Übungsordner**. `..`, absolute Pfade und
Laufwerksbuchstaben werden abgewiesen — das ist eine bindende
Sicherheitsgrenze (SPEC §2), keine Stilfrage.

## 4. Wann welcher Check — die wichtigste Entscheidung

**Regel (ADR 0005): `antwort` nur für geschlossenes Vokabular.**

| Die richtige Antwort ist … | dann |
|---|---|
| für alle gleich, ein bis zwei Wörter (`gpt`, `mirror`, `2`, `inkrementell`) | `antwort` mit Hashes |
| maschinenabhängig (Kernzahl, RAM, Firmware-Typ, Seriennummer, eigener Hashwert) | `alle_antworten` — nur Vorhandensein |
| Freitext, Begründung, Reflexion | `alle_antworten` — Bewertung macht der Trainer |
| zwei eigene Werte, die übereinstimmen müssen | `werte_gleich` |
| „hat der Lernende die Arbeit wirklich getan" | `file_matches` auf ein Capture |

Warum so streng: Eine gehashte Erwartung auf einen maschinenabhängigen Wert
lässt **korrekte** Arbeit rot werden. Das ist der Vertrauensbruch, den dieses
Produkt sich nicht leisten kann.

## 5. Regex-Muster, die auf echten Windows-Systemen halten

Der Runner liest Dateien tolerant (UTF-8 → UTF-16LE → CP850). Um die
Lokalisierung muss sich das Muster selbst kümmern.

- **Eigenschaftsnamen bleiben englisch.** `NumberOfCores`, `PartitionStyle`,
  `RealTimeProtectionEnabled` — daran darf man sich festhalten.
- **Werte und Programmtexte sind lokalisiert.** Immer als Alternation schreiben:
  `(?i)(Healthy|Fehlerfrei)`, `(?i)(Protection On|Schutz ist aktiviert)`,
  `(?i)(Copied|Kopiert)`.
- **Besser als Wörter: selbst gesetzte Dateinamen.** In Übung 04 prüfen wir
  `rechnung-002` im inkrementellen Protokoll statt der robocopy-Tabelle, deren
  Spaltenköpfe in *beiden* Läufen gleich aussehen.
- **Noch besser: strukturelle Muster.** `\{[0-9a-fA-F-]{36}\}` für eine GUID,
  `(?m)^\s*0\s*$` für eine gezählte Null. Locale-fest und nicht zufällig
  erfüllbar.
- **Keine Negation.** Die verwendete Regex-Bibliothek kennt kein Lookaround.
  „Ordner ist weg" wird positiv bewiesen:
  `Test-Path … | Out-File abgabe\aufraeumen.txt` → `(?i)(False|Falsch)`.
- **PowerShell-Booleans sind kulturunabhängig** (`True`/`False`). `|Falsch` im
  Muster ist reine Vorsicht.
- **Vorsicht bei Teilwörtern.** `(?i)Locked` trifft auch `Unlocked`. Dann mit
  Kontext prüfen: `(?i)LockStatus\s*:\s*(Locked|Gesperrt)`.

Kontrollfrage vor jedem `file_matches`: *Kann dieses Muster auf einem korrekt
gelösten System fehlschlagen?* Wenn ja: umbauen.

## 6. Hashes erzeugen

```sh
cargo run -q --manifest-path runner/Cargo.toml -- \
    intern hash --salt wb1:gt:09 "gpt" "guid partition table"
```

Ausgabe ist ein fertiger `expect_hash`-Block zum Einsetzen.

- Salt je Übung, Schema `wb1:<modul-kurz>:<nummer>`. Nie zwei Übungen mit
  demselben Salt.
- Normalisierung: `trim` → `lowercase` → innere Leerzeichen zusammenfassen.
  Deshalb braucht **jede Schreibweise einen eigenen Hash** — auch `prüfsumme`
  und `pruefsumme`, auch `aufräumen` und `aufraeumen`. Umlaute werden nicht
  ersetzt.
- Großschreibung, Leerzeichen am Rand und doppelte Leerzeichen sind schon
  abgedeckt. Dafür braucht es keine Extra-Hashes.
- `wb intern hash` gibt **nur** Hashes aus. Der Klartext erscheint nie in stdout
  und kann so nicht aus Versehen in einen Commit wandern.

## 7. Lösungen — außerhalb dieses Repos

Für jede Übung eine `LOESUNG.md` im **privaten** Repo `werkbank-loesungen`:

```
geraetetechnik/09-beispiel/LOESUNG.md
```

Inhalt: vollständiger Befehlsweg, erwartete Ausgaben, **alle** akzeptierten
Klartext-Antworten mit ihren Hashes, die genutzten `wb intern hash`-Aufrufe,
typische Fehler, didaktische Absicht.

Verboten in `werkbank` (CLAUDE.md Regel 6): Klartext-Antworten in Kommentaren,
in Tests, in Fixtures, in `trainer/`, in Beispieldateien unter `material/`.
Auch nicht „auskommentiert".

## 8. AUFGABE.md — Aufbau und Sprache

Reihenfolge, die alle acht Übungen dieses Moduls einhalten:

1. Titel + **Ziel in einem Satz** (fett)
2. Zeit · Schwierigkeit · LB-Relevanz
3. **Vorher** — Snapshot-Hinweis, `cd`, `$a`-Variable
4. **Schritte** — numeriert, jeder Befehl kopierfertig
5. **Falls du Windows 11 nutzt** — nur wo es wirklich abweicht
6. **Aufräumen** — mit Beweis
7. **Abgabe** — welche Dateien am Ende da sein müssen
8. **KI-Stufe** — was erlaubt ist und warum
9. **Reflexion** — eine Frage, keine Aufgabe
10. **Bonus** / **Homelab** — freiwillig, klar gekennzeichnet

Sprache:

- Deutsch, Niveau ~B1. Kurze Sätze. Zielgruppe sind Anfänger in einer
  gesundheitsbedingten Umschulung, nicht Admins.
- Fachbegriffe bei der ersten Verwendung erklären: Cmdlet, Pipe, Snapshot,
  Speicherpool, Schattenkopie, Hash.
- Domänenbegriffe bleiben deutsch: Übung, Abgabe, Fortschritt, Bericht,
  Vertiefung.
- **Nie beschämend.** Kein „eigentlich trivial", kein „ganz einfach". Statt
  „falsch" lieber „noch nicht" — und immer den nächsten Schritt nennen.
- Anfänger tippen keine langen Pfade. Deshalb `$a` in jeder Übung, und deshalb
  liegt in `material/` eine `antworten-vorlage.toml` zum Kopieren: das verhindert
  TOML-Syntaxfehler, den häufigsten mechanischen Frust.
- Hinweise (`hint_de`) sagen **wohin schauen**, nie **was eintragen**. Ein guter
  Hinweis nennt den Befehl, mit dem der Lernende es selbst herausfindet.

## 9. Prüfen, bevor es eingecheckt wird

```sh
cargo run -q --manifest-path runner/Cargo.toml -- intern lint uebungen
cargo test --manifest-path runner/Cargo.toml
```

`intern lint` prüft Schema, Pfad-Ausbrüche, Regex-Kompilierung, doppelte
Check-IDs, Hash-Format und dass mindestens ein Basis-Check existiert. Der Lint
läuft in CI: **eine kaputte `exercise.toml` macht die Pipeline rot.** Inhalt ist
Code.

Danach von Hand: Übung selbst lösen, `wb check` grün sehen, `wb status` und
`wb bericht` ansehen. Eine Übung, die der Autor nicht selbst gelöst hat, geht
nicht in eine Klasse.

## 10. Prüfungsintegrität und IP

- Kein Text aus Prüfungsunterlagen, keinem Trägermaterial, keiner fremden
  Wissensdatenbank. Auch nicht umformuliert. Übungen trainieren dieselbe
  Kompetenz mit **anderen Parametern**.
- Beispieldaten sind erfunden. Keine echten Firmen, keine echten Personen, keine
  echten Adressen. (Hostnamen wie `aquinas` oder `edith-stein` sind erfunden und
  gewollt — siehe `SOLI_DEO_GLORIA.md`.)
- Externe Quellen (Microsoft Learn, CompTIA) werden **verlinkt**, nie
  wiedergegeben.
- Lernenden-Inhalte bleiben konfessionell neutral (SPEC §7).
