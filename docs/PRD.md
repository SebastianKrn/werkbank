# PRD — Werkbank (Arbeitstitel)

> Praxis-Layer für IT-Umschulungen. Pilotmodul: **Gerätetechnik** (BBRZ Wien, Klassenraum Raphael).
> Stand: 2026-07-25 · Owner: Sebastian Kern · Trainer/Pilot: Raphael Lugmayr

## 0. Entscheidungszusammenfassung

| Entscheidung | Ergebnis | Begründung |
|---|---|---|
| Pilotthema | **Gerätetechnik** | Raphaels eigener Klassenraum (Sebastian supplert) — null Genehmigungsrisiko, direkter Feedback-Loop mit echten Lernenden. |
| Delivery-Modell | **Local-first Check-Runner** | Zielgruppe: absolute Beginner, gesundheitsbedingte Umschulung, potenziell gesperrte BBRZ-PCs. Ein Befehl → sofortiges deutsches Feedback. Offline-fähig, keine Accounts, keine CI-Abhängigkeit. |
| Monetarisierung | **Keine im MVP.** Institutionell später (Trainer-Tagsätze, Content-Partnerschaften). | Affiliate-Modell ist tot (Hetzner-Programm eingestellt, OpenAI/Anthropic haben keine). Zahler im Markt sind Träger (AMS-finanziert), nicht Lernende. |
| Open Source | Runner: ja (MIT/Apache-2.0). Content: ja, Lizenz offen (siehe §10). Lösungen: privat. | Repo = Glaubwürdigkeit, Vortrags-Grundlage, SEO. Lösungen getrennt, sonst ist jede Übung wertlos. |
| Kein Code vor M0-Gate | **M0 grün (bedingt) seit 2026-07-25**: externes Material erlaubt; Umgebung = Win11-Laptops (32 GB) + lernenden-administrierte Win11/Server-VMs je Übung; EXE machbar. | Gleiche Disziplin wie digitales-nest: erst validieren, dann bauen. M1 darf starten; M2 wartet auf LB-Themenliste. |
| Pilot-Fokus | **Vorbereitung auf die anstehende „Leistungsbeurteilung" (LB)** in Raphaels Gruppe (~7 Personen). LB liegt vor (2026-07-25): 100-min-Praxisprüfung am Windows Server 2022 (QEMU-VM, kein TPM) — Szenario, Hardware-Inspektion, RAID 1 (Storage Spaces), SHA-256, robocopy-Backups, VSS, Ausfall-Simulation, BitLocker, Reflexion, Aufräumpflicht. | Konkreter, terminierter Bedarf schlägt generisches Curriculum. 8 Übungen mappen 1:1 auf die LB-Kompetenzen (SPEC §4), aber **umparametrisiert** — Kompetenztraining, keine Prüfungskopie. Ziel: auch die Schwächsten bestehen sicher. |
| Name & Lizenz | **Final: „Werkbank"**; Content CC BY-NC-SA 4.0, Runner MIT OR Apache-2.0 | Vom Owner bestätigt 2026-07-25. |
| Heterogenität | Drei Stufen je Übung: **Basis** (LB-relevant, Pflicht) / **Bonus** (Vertiefung) / **Homelab** (freiwillig, z. B. Raspberry Pi) | Gruppe reicht von Ex-Hotellerie/Altenpflege ohne Grundlagen bis Homelab-Besitzer. Eine Übung, drei Tiefen — niemand langweilt sich, niemand ertrinkt. |

## 1. Problem

Menschen, die aus gesundheitlichen Gründen in die IT umschulen (BBRZ-Kernklientel), scheitern nicht am fehlenden Content — Content ist im Überfluss gratis vorhanden (Cisco NetAcad, CS50, freeCodeCamp, Microsoft Learn). Sie scheitern an fehlender **Struktur**: kein klarer nächster Schritt, kein sofortiges Feedback, kein „du bist hier". Referenzfall 42 Wels: große PDF-Aufgaben + „frag die KI oder deinen Nachbarn" → Abbruch genau der Menschen, die Werkbank erreichen will. Auf Trainerseite baut jede:r BBRZ-Fachexpert:in (21 Themen, u. a. Gerätetechnik, Linux, C#, Netzwerktechnik) Materialien solo — es gibt keine gemeinsame Übungs-Infrastruktur mit automatischem Feedback.

**Kosten des Nicht-Lösens:** Abbrüche in AMS-finanzierten Maßnahmen, Trainer-Zeit fließt in Materialbau statt Betreuung, Lernende üben zu wenig eigenständig — und lernen im KI-Zeitalter nie den Unterschied zwischen „selbst können" und „KI fragen".

## 2. Lösung

Ein **Praxis-Layer**, der neben jeden bestehenden Kurs gelegt werden kann:

1. **Übungspakete** (Repo oder ZIP): pro Übung ein Ordner mit `AUFGABE.md` (Deutsch, einfache Sprache), Startdateien und deklarativen Checks.
2. **Check-Runner `wb`** (ein portables Binary, keine Installation, keine Adminrechte): Lernende:r arbeitet, tippt `wb check`, bekommt sofort deutsches Feedback mit Hinweis statt Lösung. Fortschritt lokal, Bericht für Trainer per `wb bericht`.
3. **Trainer-Handbuch** pro Modul: Ablaufplan, typische Stolperstellen, Lösungen (privat).
4. **KI-Didaktik eingebaut:** Jede Übung hat eine markierte KI-Stufe („erst selbst, dann KI") + Reflexionsfragen. Nicht technisch erzwungen — didaktisch verankert.

Das Format ist themenagnostisch. Pilot-Inhalt: **Modul Gerätetechnik** (8 Übungen, 1:1 auf die LB-Kompetenzen A1–A8+R gemappt und umparametrisiert; CompTIA A+ Core 1 und Microsoft-Learn-Module als kostenlose Vertiefungslinks).

## 3. Warum wir / warum jetzt

- **Raphael steht im Klassenraum.** Aktiver BBRZ-Trainer für Gerätetechnik mit eigener Wissensdatenbank; Sebastian supplert. Pilotgruppe existiert, bevor eine Zeile Code existiert.
- **Sebastian will lehren.** Werkbank ist das wiederverwendbare Fundament für jede künftige Lecture (FH, WIFI, BFI, JKU-Kontakt Plösch).
- **Der 42-Referenzfall** liefert die Design-These frei Haus: Struktur schlägt Content.
- **IT-Fachkräftemangel in Österreich** (Cybersecurity, Cloud, DevOps als Mangelberufe) hält die Nachfrage nach Umschulungen hoch — AMS-finanziert, d. h. zahlungsfähige Träger.
- **Plösch-Anker:** Value-Based Requirements Prioritization fließt als Denkschule in fortgeschrittene Module ein („baue, was Wert bringt") — Differenzierung + akademischer Gesprächsanker.

## 4. Zielgruppen & Personas

1. **„Der Umschüler" (primär, Pilot):** 30–55, gesundheitsbedingter Berufswechsel (real in der Pilotgruppe: Ex-Hotellerie, Ex-Altenpflege), lernt in BBRZ-Präsenzgruppe (~7 Personen). **Stark heterogen:** von „fehlende Basics, geringes Selbstvertrauen" bis „eigenes Homelab". Braucht: kleine Schritte, sofortiges Erfolgserlebnis, deutsche einfache Sprache, keine Einstiegshürde — und nach oben offene Bonus-/Homelab-Stufen, damit die Erfahrenen gefordert bleiben. Akutes gemeinsames Ziel: die **Leistungsbeurteilung bestehen**.
2. **„Der Trainer" (sekundär, Pilot):** BBRZ-Fachexperte (Raphael). Braucht: einsetzbares Material ohne Umbau seines Unterrichts, Überblick wer wo steht, weniger Mechanik-Fragen („wo speichere ich das?").
3. **„Der Senior" (später, Phase 2):** Entwickler:in wie Sebastian, will Grundfertigkeiten trotz KI-Alltag halten. Eigene Modullinie („ohne KI bauen, dann mit KI verbessern"). **Nicht im MVP.**

## 5. Scope

### Phase 0 — Validierung vor Software (M0, kein Code)
- Raphael klärt BBRZ-Rahmen: Darf externes Übungsmaterial im Unterricht eingesetzt werden? (Seine Wissensdatenbank existiert bereits → Präzedenzfall, trotzdem sauber absichern.)
- Klassenraum-Constraints-Checkliste: Windows-Version, Adminrechte, Netzzugang, USB-Policy, vorhandene Tools.
- Pilotgruppe + Zeitfenster fixieren.

### MVP (M1–M4, siehe MILESTONES.md)
- Runner `wb` (Rust, ein Binary, Windows-first) mit Check-Engine, deutschem Feedback, Fortschritt, Bericht.
- Modul Gerätetechnik: **8 Übungen** (Nr. 8 = „Generalprobe": eigenes Mini-Szenario end-to-end + Reflexion + Aufräumen), gemappt auf die LB-Aufgaben A1–A8+R (SPEC §4), Trainer-Handbuch, private Lösungen. Jede Übung endet mit eigenem Aufräum-Schritt (LB-Gewohnheit trainieren).
- Übungen laufen **in den Lernenden-VMs** (Win11/Server, Adminrechte vorhanden): echte Captures statt gestellter Exporte, wo sinnvoll; VM-Snapshot als „Reset-Knopf" didaktisch genutzt.
- **Stufenmodell** je Übung: Basis (Pflicht, LB-relevant) / Bonus / Homelab (freiwillig, z. B. Raspberry-Pi-Varianten).
- Verteilbares ZIP (kein Git-Zwang; Git ist Lernziel, nicht Zugangshürde).
- Pilotdurchlauf mit Messung (Metriken §8) und Retro.

### Explizit NICHT im MVP
- **Kein Server, keine Accounts, keine Web-App, keine Datenbank.** (Größte Versuchung, größter Zeitfresser.)
- Keine weiteren Module (Linux, C#, Netzwerktechnik) — erst nach Pilot-Gate.
- Kein Senior-Track.
- Keine CI-/Cloud-Auswertung, kein GitHub Classroom.
- Keine LMS-/Moodle-Integration.
- Keine Monetarisierung, keine Affiliate-Links, kein „Buy me a coffee".
- Keine ID-Austria-Integration (kein realer Use Case im Übungskontext des Piloten).
- Kein Kopieren von BBRZ-/Wissensdatenbank-Inhalten (IP-Hygiene, siehe §9).

## 6. User Stories (Auszug, priorisiert)

- Als **Umschüler** will ich eine Übung mit einem einzigen Befehl prüfen können, damit ich sofort weiß, ob ich richtig liege — ohne auf den Trainer zu warten. *(P0)*
- Als **Umschüler** will ich bei einem Fehler einen Hinweis statt der Lösung bekommen, damit ich es selbst schaffen kann. *(P0)*
- Als **Umschüler** will ich jederzeit sehen, wo ich stehe (`wb status`), damit ich nicht das 42-Gefühl „verloren im PDF" habe. *(P0)*
- Als **Umschüler** will ich ohne Installation und ohne Adminrechte starten können (ZIP entpacken → loslegen), damit die Technik mich nicht am ersten Tag besiegt. *(P0)*
- Als **Trainer** will ich pro Lernendem einen kompakten Bericht (`wb bericht` → Datei/Text zum Abgeben), damit ich Betreuung priorisieren kann. *(P0)*
- Als **Trainer** will ich im Handbuch typische Stolperstellen je Übung sehen, damit Supplierende (z. B. Sebastian) ohne Vorlauf unterrichten können. *(P1)*
- Als **Umschüler** will ich nach jeder Übung eine kurze Reflexionsfrage zur KI-Nutzung beantworten, damit ich lerne, wann KI hilft und wann sie mir das Lernen stiehlt. *(P1)*
- Als **Content-Autor** (Sebastian) will ich eine neue Übung nur durch Anlegen eines Ordners mit `exercise.toml` erstellen können, damit weitere Module ohne Runner-Änderung entstehen. *(P0, Architektur)*

## 7. Nicht-funktionale Anforderungen

1. **Zero-Install:** Läuft aus entpacktem ZIP auf Windows 10/11 ohne Adminrechte, ohne Runtime (kein Python/Node/JVM). Linux/macOS-Builds zusätzlich.
2. **Offline-first:** Alle Checks lokal. Internet nur für optionale Vertiefungslinks.
3. **Sprache:** Lernenden-Output 100 % Deutsch, einfache Sprache (~B1), keine Anglizismen ohne Erklärung. Fehlermeldungen ermutigend, nie beschämend.
4. **Robustheit:** Tolerant gegenüber Windows-Encodings (UTF-8/UTF-16/CP850) und lokalisierten Befehlsausgaben (deutsche `systeminfo`-Labels).
5. **Barrierearm:** Klare Struktur, keine Farb-only-Information im Terminal-Output, Doku-PDFs mit ausreichender Schriftgröße (Zielgruppe inkludiert gesundheitliche Einschränkungen).
6. **Datenschutz:** Keine Telemetrie, keine personenbezogenen Daten im Repo. Fortschritt bleibt lokal beim Lernenden; Bericht wird bewusst vom Lernenden übergeben. (Vulnerable Zielgruppe → Datenminimierung by design.)
7. **IP-Hygiene:** Alle Inhalte original. Mapping auf A+/Microsoft Learn nur als Verweis, keine Übernahme geschützter Inhalte.

## 8. Erfolgs-Metriken (Pilot-Gate für Ausbau)

| Metrik | Ziel | Stretch |
|---|---|---|
| Lernende, die Übung 1 ohne Mechanik-Hilfe starten (ZIP → `wb check` erfolgreich) | ≥ 80 % | 100 % |
| Lernende mit ≥ 5 von 8 Übungen bestanden bis Pilotende | ≥ 60 % | ≥ 80 % |
| **LB-Ergebnis der Gruppe (True North)**: alle Teilnehmenden bestehen (Note 1–4) | 7/7 bestehen | Notenschnitt ≤ 3 |
| Runner läuft auf BBRZ-PCs beim ersten Versuch | ja | — |
| Trainer-Urteil Raphael: „setze ich nächste Gruppe wieder ein" | ja | „Kollegen fragen danach" |
| Beta-Test 42-Freund: Modul solo abschließbar | ja | positives O-Ton-Zitat |
| Mechanik-Fragen an Trainer („wo speichern?", „wie prüfen?") | spürbar ↓ ggü. normalem Unterricht (qualitativ) | — |

Messmethode: `wb bericht`-Abgaben + kurzes Papier-/Forms-Feedback + Trainer-Retro. Auswertung 1 Woche nach Pilotende.

## 9. Risiken & Gegenmaßnahmen

| Risiko | Wirkung | Gegenmaßnahme |
|---|---|---|
| BBRZ untersagt/limitiert externes Material | Pilot platzt | M0-Gate vor jeder Codezeile; Raphaels bestehende Wissensdatenbank als Präzedenzfall; notfalls Pilot mit 42-Freund + privaten Testern |
| Interessenkonflikt Raphael (Angestellter pilotiert Stoicera-Produkt) | Vertrauensschaden, blockiert spätere Träger-Deals | Transparenz gegenüber BBRZ-Leitung in M0; Pilot ausdrücklich unentgeltlich; Inhalte original (nichts aus BBRZ-Material) |
| Gesperrte Klassen-PCs blockieren sogar portable EXE (AppLocker o. ä.) | Runner läuft nicht | M0-Checkliste klärt das vorab; Fallback: Checks als reine PowerShell-Skripte (signierbar) — Entscheidung in M0, nicht später |
| Übungen zu schwer/zu leicht für Zielgruppe | Frust/Langeweile, Metriken scheitern | Beta-Test mit 42-Freund vor Klasseneinsatz (M3); Schwierigkeitsgrade + Bonusaufgaben |
| Scope-Explosion Richtung Plattform („nur noch schnell eine Web-UI…") | MVP verfehlt Pilottermin | Harte NICHT-Liste §5; CLAUDE.md-Regel; Milestone-Disziplin |
| Lösungen leaken (Antworten im Klartext) | Übungen entwertet | Erwartete Antworten nur als salted Hashes im Übungspaket; Lösungen im privaten Repo |
| Themenexperten-Egos („mein Fach, mein Material") beim späteren Ausbau | Skalierung stockt | Pilot-Ergebnisse sprechen lassen; Format als Angebot an Experten, nicht Ersatz |

## 10. Offene Punkte / Entscheidungen an den Owner

1. ~~**Name**~~ **ENTSCHIEDEN 2026-07-25: „Werkbank".**
2. ~~**Content-Lizenz**~~ **ENTSCHIEDEN 2026-07-25: Content CC BY-NC-SA 4.0, Runner MIT OR Apache-2.0.**
3. **BBRZ-Formalisierung:** ✅ Externes Material erlaubt (M0, 2026-07-25).
4. **Wissensdatenbank-Zugang:** Optional. LB-Dokument liegt vor und ist die maßgebliche Content-Quelle; Wissensdatenbank nur noch als Stil-/Begriffsabgleich interessant (referenzieren, nie kopieren).
5. **Pilottermin:** Datum fixieren — der Pilot muss **vor der Leistungsbeurteilung** abgeschlossen sein (M3-Deadline rückwärtsrechnen). **Einziger offener Blocker neben Beta-Tester-Zusage.**
6. ~~**LB-Unterlagen**~~ ✅ Erhalten 2026-07-25 (Praktische Leistungsüberprüfung, 100 min, A1–A9 + R). Mapping in SPEC §4 eingearbeitet. Dokument bleibt außerhalb des Repos (Prüfungsintegrität + IP, SPEC §7).
7. **Glaubens-Ausdruck (Owner-Entscheidung, Empfehlung dokumentiert):** Sebastian ist Katholik und will Gott als eigentlichen Urheber ehren. Empfehlung: **Lernpfad im BBRZ-Einsatz bleibt konfessionell neutral** — die Gruppe ist eine schutzbedürftige, konfessionell gemischte Pflichtmaßnahme, und offen religiöse Übungsinhalte könnten Raphaels Stellung und spätere Träger-Deals gefährden. Der Glaube bekommt stattdessen ehrliche, nicht-vereinnahmende Orte: `SOLI_DEO_GLORIA.md` im Repo-Root (Widmung/Kolophon), verstecktes Easter-Egg-Kommando `wb deo-gratias`, dezente Anspielungen in erfundenen Beispieldaten (z. B. Hostnamen wie `aquinas`, `edith-stein`), Release-Namen nach Heiligen. Ein eigenes optionales Modul mit Bibel-/Katechismus-Bezug („Glaubenswerk", z. B. Textverarbeitung/Regex-Übungen an Psalmtexten) ist als separates, klar gekennzeichnetes Zusatzpaket für die eigene Community (Sword and Seed, Christlichdenken) sinnvoll — nicht im BBRZ-ZIP.
