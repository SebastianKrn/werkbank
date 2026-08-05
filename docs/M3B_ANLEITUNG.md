# M3b — deine Handarbeit, der Reihe nach

**Das ist die Seite, die du zuerst aufmachst.** Sie sagt dir, was zu tun ist,
in welcher Reihenfolge, wie lange es dauert und in welchem Dokument die Details
stehen. Nichts davon kann eine Programmiersitzung erledigen — jeder Punkt
braucht einen Menschen, eine echte Windows-VM oder eine andere Person.

Stand: 2026-08-02.

---

## 1 · Wo das Projekt steht

| Meilenstein | Stand |
|---|---|
| M0 Validierungs-Gate | **grün** (2026-07-25) |
| M1 Runner | **fertig** (2026-07-25) |
| M2 Inhalte Gerätetechnik | **fertig** (2026-07-26) — offen bleibt nur Raphaels Curriculum-Review |
| M3a Freeze-Maschinerie | **fertig** (2026-07-26) |
| **M3b Trockenläufe** | **← du bist hier** |
| M4 Pilot | gesperrt bis M3b durch ist |

**Das Testobjekt:** `v0.1.0-rc2`, veröffentlicht am 2026-07-27.
<https://github.com/SebastianKrn/werkbank/releases/tag/v0.1.0-rc2>

Gebaut aus `main` nach dem Härtungs-Review, auf beiden Plattformen
test-abgesichert. **Noch kein Mensch hat es auf Windows ausgeführt.** Genau das
ist Schritt 1.

### Was aus dem Review schon erledigt ist

Damit du es nicht noch einmal aufmachst — diese vier Punkte aus deinen
Notizen sind **geschlossen**:

| Punkt | Erledigt in |
|---|---|
| PR #3 prüfen und mergen | gemergt am 2026-07-27 |
| GitHub-Actions auf Commit-SHAs festnageln | `8e3549b` |
| ADR für die deutsch-englische Schema-Sprache | `efcdb43` (ADR 0007) |
| Junction-Test für `wb check` | `1a584a0`, korrigiert in `e0341dc` — läuft auf windows-latest in der CI |

Vom „Junction-Test fehlt" bleibt eine **kleinere** Lücke übrig: der rekursive
Durchlauf von `wb erfasse ordnerliste` ist nur auf Unix getestet. Das ist Teil F
des Protokolls, nicht mehr ein eigener Punkt.

---

## 2 · Die vier Schritte

**Streng in dieser Reihenfolge.** Schritt 2 ist wertlos, wenn Schritt 1 einen
Blocker gefunden hat: du würdest deinen einzigen unvoreingenommenen Testleser
an ein kaputtes ZIP verbrennen.

| # | Schritt | Dauer | Anleitung |
|---|---|---|---|
| 1a | Test-VM bauen | ~90 min | `docs/VM_WINDOWS_SERVER.md` |
| 1b | Testprotokoll durchlaufen | 2–3 h | `docs/TESTPROTOKOLL.md` |
| 1c | Funde beheben, `v0.1.0-rc3` bauen, Teil A+B wiederholen | 1–3 h | `docs/RELEASE.md` |
| 2 | Externe Beta (42-Freund) | 30 min deine Zeit | unten, Abschnitt 2.2 |
| 3 | Probeunterricht mit Raphael + sein Curriculum-Review | 90 min | unten, Abschnitt 2.3 |
| 4 | Freeze: Tag `v0.1.0`, Handzettel drucken | 30 min | `docs/RELEASE.md`, Abschnitt „Der Freeze" |

### 2.1 Schritt 1 — Der Testprotokoll-Lauf

**Voraussetzungen, bevor du anfängst:**

- [ ] Virtualisierung im BIOS eingeschaltet — **auf deinem Notebook ist sie
      aktuell aus**, siehe `docs/VM_WINDOWS_SERVER.md` Schritt 0
- [ ] ~80 GB freier Plattenplatz
- [ ] Windows-Server-2022-Evaluierungs-ISO **auf Deutsch** heruntergeladen

**Fang mit Teil C an.** Sieben der neun Erfassen-Presets sind noch nie
irgendwo gelaufen, und zwei davon können auf Linux gar nicht laufen. Wenn der
Pilot bricht, bricht er dort.

**Fertig, wenn:** Teile A–F durch sind, alle Blocker behoben, `v0.1.0-rc3`
gebaut und die Teile A und B dagegen sauber wiederholt.

### 2.2 Schritt 2 — Die externe Beta

Erst starten, wenn Schritt 1 sauber ist.

**Schick genau drei Dinge, und nichts sonst:**

1. den Link zum Release
2. `trainer/BETA_FEEDBACK.md` (als Datei oder ausgedruckt)
3. den Satz: **„Frag niemanden. Auch mich nicht."**

**Beantworte keine einzige Frage von Hand.** Genau eine beantwortete Frage
zerstört die Messung — gemessen wird, ob das Modul einen Menschen allein trägt,
nicht ob du ihn tragen kannst.

**Fertig, wenn:** der Bogen zurück ist. Messlatte laut Definition of Done:
**mindestens 6 von 8 Übungen ohne mechanische Hilfe geschafft.**

### 2.3 Schritt 3 — Probeunterricht mit Raphael

Zwei Dinge in einem Termin, weil beide Raphael brauchen:

- **Sein Curriculum-Review** (30 min) — offen seit M2. Passt die Übungsliste zu
  dem, was er unterrichtet? Das ist seine Entscheidung, nicht unsere.
- **Probeunterricht** (60 min) — du unterrichtest ihm **eine** Übung als
  Rollenspiel. Was du dabei erklären musst, obwohl es im Handbuch stehen
  sollte, wanderte anschließend in `trainer/HANDBUCH_GERAETETECHNIK.md`.

**Bring mit:** deine gemessenen Zeiten aus Teil D des Protokolls. Der
4-Block-Plan im Handbuch beruht bis dahin auf Schätzungen.

### 2.4 Schritt 4 — Freeze

`docs/RELEASE.md`, Abschnitt „Der Freeze". Ein Tag, mehr ist es nicht.

**Harte Bedingung:** Der Freeze muss **mindestens 3 Tage vor dem Pilottag**
liegen, und der Pilot vor dem LB-Termin.

---

## 3 · Drei Entscheidungen

Alle drei sind deine. Keine hat eine Frist außer der ersten — und auch die nur,
wenn du sie überhaupt für nötig hältst. Lies die Empfehlung, kreuz an, fertig.

### 3.1 Die geleakten Antwort-Wörter

**Die Frage:** In der Git-Historie stehen dauerhaft 11 akzeptierte
Antwortwörter, aus den Übungen 02, 03, 04, 06, 07 und 08. Aus den Dateien sind
sie entfernt (`9c4e565`), aus der Historie nicht — dort bleiben sie für immer.

**Die Fakten, ohne Beschönigung:**

- **Neu salzen behebt das nicht.** Neue Hashes machen die 11 Wörter in der
  Historie nicht ungültig — die Wörter sind weiterhin die richtigen Antworten.
  Nur eine **Umparametrisierung** dieser Fragen würde das ändern, und das ist
  Inhaltsarbeit an sechs Übungen.
- **ADR 0003 akzeptiert das Problem bereits.** Kurze Antworten aus geschlossenem
  Vokabular sind rechenbar. Wer eine Liste von 50 Fachwörtern durchprobiert,
  hat sie in unter einer Sekunde — mit oder ohne Leak.
- **Neu salzen ohne `../werkbank-loesungen` ist gefährlich.** Dieses Repo kennt
  nur Hashes. Ein teilweiser Neu-Salz-Lauf zerstört akzeptierte Schreibweisen
  unwiederbringlich.

**Empfehlung: nichts tun.** Der Aufwand kauft Disziplin, keine Sicherheit. Halt
es stattdessen in ADR 0003 fest, damit es später niemand als Versehen liest.

☐ nichts tun (empfohlen) ☐ neu salzen ☐ die sechs Fragen umparametrisieren

Falls neu salzen: die Befehle stehen in `../werkbank-loesungen/README.md`,
Abschnitt „Hashes neu erzeugen". Jede akzeptierte **Schreibweise** braucht einen
eigenen Hash-Eintrag, auch `ß` gegen `ss`.

### 3.2 Darf das öffentliche Trainer-Handbuch Antworten enthalten?

**Die Frage:** `trainer/HANDBUCH_GERAETETECHNIK.md` nennt Fachbegriffe, die es
unterrichten soll — und mindestens einer davon ist gleichzeitig eine
akzeptierte Antwort in Übung 03. CLAUDE.md Regel 6 sagt: keine Lösungen in
diesem Repo. Das ist ein echter Widerspruch, kein Formfehler.

(Welche Begriffe das sind, steht bewusst nicht hier — dieses Dokument ist
öffentlich und soll die Liste nicht verlängern.)

**Die Fakten:**

- `trainer/` erreicht das Lernenden-ZIP **nie**. `scripts/paket.sh` hat dafür
  einen Stolperdraht, der den Build abbricht (Exit 3) — das ist mechanisch
  erzwungen, nicht Disziplin.
- Das Repo ist öffentlich. Wer will, liest das Handbuch auf GitHub.
- Ein Trainer-Handbuch, das den Begriff nicht nennen darf, den es erklären soll,
  ist unbrauchbar.

**Empfehlung: ausdrückliche Ausnahme für `trainer/`.** Regel 6 schützt das ZIP,
und das ZIP ist mechanisch geschützt. Schreib die Ausnahme in CLAUDE.md hin,
statt sie stillschweigend zu praktizieren — Regeln, die man stillschweigend
bricht, hören auf zu wirken.

☐ Ausnahme für `trainer/` in CLAUDE.md (empfohlen) ☐ Handbuch ins private Repo

### 3.3 Code-Signierung und Herkunftsnachweis

**Die Frage:** Lernende sollen an einer SmartScreen-Warnung vorbeiklicken.
Lohnt ein Zertifikat?

**Die Fakten — nachgeschlagen bei Microsoft, Stand 2026-08-02:**

| Weg | Kosten | Für dich möglich? | Wirkung auf SmartScreen |
|---|---|---|---|
| Azure Artifact Signing (früher Trusted Signing) | ~9,99 $/Monat | **Nein.** Einzelentwickler nur USA/Kanada. Als Firma bräuchte es 3+ Jahre nachweisbare Historie. | baut erst über Wochen Reputation auf |
| OV-Zertifikat (DigiCert, Sectigo) | 150–300 $/Jahr + Hardware-Token | ja | baut erst über Wochen Reputation auf |
| EV-Zertifikat | 400+ $/Jahr | ja | **umgeht SmartScreen seit 2024 nicht mehr** |
| SignPath Foundation | kostenlos für Open Source | vielleicht — Antrag nötig | wie OV |
| GitHub Build-Provenance | kostenlos | ja, sofort | **keine** — beweist Herkunft, keine Signatur |

**Der entscheidende Punkt:** Alle bezahlten Wege bauen Reputation *pro Datei*
über Wochen und hunderte saubere Downloads auf. Dein Pilot hat **sieben
Lernende**. Diese Reputation entsteht nie. Du zahlst für dieselbe Warnung.

**Empfehlung: kein Zertifikat für den Piloten.** Stattdessen kostenlose
Build-Provenance einschalten — sie macht überprüfbar, dass das ZIP aus genau
diesem Repo und genau dieser Pipeline stammt. Und: `START_HIER.md` erklärt die
Warnung bereits in einem ruhigen Ton, was in dieser Lage mehr wert ist als
Geld.

☐ nichts kaufen + Provenance einschalten (empfohlen) ☐ nur nichts kaufen
☐ SignPath Foundation beantragen ☐ OV-Zertifikat kaufen

**Falls Provenance:** In `.github/workflows/release.yml` beim Job `package`:

```yaml
  package:
    name: package and verify
    needs: [guard, build]
    runs-on: ubuntu-latest
    permissions:            # <- neu, der Job hat sonst nur contents: read
      contents: read
      id-token: write
      attestations: write
    steps:
      # ... alles Bestehende bleibt, danach als letzter Schritt:
      - name: Attest build provenance
        uses: actions/attest-build-provenance@0f67c3f4856b2e3261c31976d6725780e5e4c373 # v4.1.1
        with:
          subject-path: dist/werkbank-${{ env.MODUL }}-${{ needs.guard.outputs.version }}.zip
```

Prüfen lässt sich das Ergebnis dann von jedem:

```bash
gh attestation verify werkbank-geraetetechnik-v0.1.0.zip -R SebastianKrn/werkbank
```

Der SHA ist festgenagelt wie alle anderen Actions im Repo (`8e3549b`).

---

## 4 · Was bewusst nicht gemacht wird

Damit es niemand für vergessen hält:

| Nicht gemacht | Warum |
|---|---|
| Windows-11-Gegenprobe für die sechs Übungen mit „Falls du Windows 11 nutzt" (03–08) | Die LB läuft auf Server 2022. Die Windows-11-Hinweise betreffen nur Lernende, die freiwillig anders arbeiten. Echte Befehlsunterschiede stehen nur in 04 (VSS) und 07 (BitLocker) — dafür ca. 90 min. Anhang im Testprotokoll, wenn du es willst. |
| Vorbeugende Windows-Korrekturen vor dem Protokoll | Ohne Beobachtung geraten. Raten macht genau die Beweise kaputt, für die das Protokoll existiert. |
| Zweites Modul, Plattformfunktionen, Monetarisierung | Gesperrt bis zum M4-Gate. |

---

## 5 · Was du in die nächste Arbeitssitzung mitbringst

Vier Dinge. Ohne sie kann die nächste Sitzung nur raten.

1. **Die Fundtabelle** (Teil G des Protokolls). Einfügen, nicht committen.
2. **Pro Fund der exakte deutsche Wortlaut** jedes Dialogs und jeder
   CLI-Meldung. Eine Umschreibung reicht nicht, um einen String zu reparieren.
3. **Welche der neun Presets** unbrauchbare Ausgabe erzeugt haben — und was
   stattdessen in der Datei stand.
4. **Deine gemessenen Zeiten** pro Übung.

Und die drei Kreuzchen aus Abschnitt 3.
