# Release bauen — Schritt für Schritt

**Wofür:** Du brauchst diese Anleitung zweimal in M3b.

1. Nach dem Testprotokoll, wenn du Blocker behoben hast → **`v0.1.0-rc3`**
2. Beim Freeze, wenn alles sauber ist → **`v0.1.0`**

**Dauer:** 5 Minuten Arbeit, danach 8–12 Minuten Warten auf die Pipeline.

**Der Grundsatz:** Das Klassen-ZIP entsteht **nur** aus einem Tag. Der
Entwicklungsrechner ist Linux und baut kein `wb.exe` (ADR 0006). Ein von Hand
zusammengebautes ZIP ist ein ZIP, bei dem irgendwann eine Ausschlussregel
vergessen wird — und eine dieser Regeln ist „keine Lösungen an Lernende".

---

## Vorher: drei Dinge müssen stimmen

### 1. Deine Änderungen sind über einen Pull Request auf `main`

```bash
git switch -c fix/protokoll-funde     # Branch, nie direkt auf main
# ... arbeiten, committen ...
git push -u origin fix/protokoll-funde
gh pr create --fill
gh pr merge --squash
```

### 2. `main` ist grün

```bash
git switch main && git pull
gh run list --branch main --limit 3
```

In der Spalte ganz links muss `completed  success` stehen. Ist sie rot, hilft
kein Tag — die Release-Pipeline testet vor dem Bauen noch einmal und bricht ab.

### 3. Tag-Name und Cargo.toml stimmen überein

```bash
grep -m1 '^version' runner/Cargo.toml
```

| Ausgabe | Erlaubte Tags |
|---|---|
| `version = "0.1.0"` | `v0.1.0`, `v0.1.0-rc1`, `v0.1.0-rc2`, `v0.1.0-rc3` … |

Ein Release-Kandidat darf die Version teilen, für die er Kandidat ist. Ein
`-rc`-Tag wird als **Vorabversion** veröffentlicht, ein Tag ohne `-rc` als
richtiges Release.

Stimmt es nicht, **erst** die Version ändern, committen und mergen — dann
taggen:

```bash
# nur falls nötig
sed -i 's/^version = .*/version = "0.1.0"/' runner/Cargo.toml
```

---

## Der Release

```bash
git switch main && git pull
git tag v0.1.0-rc3
git push origin v0.1.0-rc3
```

Das ist alles. Zuschauen:

```bash
gh run watch
```

Die Pipeline macht der Reihe nach:

| Job | Was er tut | Bricht ab, wenn |
|---|---|---|
| `version guard` | prüft Tag-Form und Abgleich mit `Cargo.toml` | Tag ist nicht `vX.Y.Z[-rcN]` oder passt nicht zur Crate-Version |
| `test before shipping` | fmt, clippy, Tests, Inhalts-Lint — auf Windows **und** Linux | irgendein Test rot |
| `build` | `wb.exe` auf windows-latest, `wb` auf ubuntu-latest | Kompilierfehler |
| `package and verify` | `scripts/paket.sh` baut das ZIP, **packt es wieder aus und führt es aus** | `wb.exe` fehlt, ZIP meldet nicht 8 Übungen, oder die Stolperdraht-Regel schlägt an (`trainer/` oder etwas Lösungsförmiges im Manifest) |
| `publish release` | hängt ZIP, `SHA256SUMS.txt`, `MANIFEST.txt` an ein GitHub-Release | — |

---

## Nachher: das Ergebnis prüfen

**Nicht überspringen.** Die Pipeline prüft das Artefakt schon selbst — dieser
Schritt prüft, dass das, was auf der Release-Seite hängt, dasselbe Artefakt ist.

```bash
cd /tmp && rm -rf release-pruefung && mkdir release-pruefung && cd release-pruefung

gh release download v0.1.0-rc3 -R SebastianKrn/werkbank
sha256sum -c SHA256SUMS.txt          # muss "OK" sagen

unzip -q werkbank-geraetetechnik-v0.1.0-rc3.zip
cd werkbank-geraetetechnik

ls -l wb.exe                          # muss da sein, ~2 MB
cat VERSION.txt                       # muss v0.1.0-rc3 sagen
./wb status                           # 8 Übungen, Fortschritt 0 von 8

# Dieselbe Stolperdraht-Regel wie in scripts/paket.sh, nur auf das ausgepackte
# ZIP angewandt: geprüft werden Pfadnamen, nicht Dateiinhalte. Ein sauberes ZIP
# gibt hier gar nichts aus.
find . | grep -Ei 'trainer/|loesung|lösung'
```

> **Warum `find` und nicht `grep -r`?** `grep -r "trainer/" .` durchsucht
> *Inhalte*. `uebungen/LICENSE` erwähnt `trainer/` in einem Satz über die
> Lizenzen und liegt in jedem ZIP — der Befehl schlug also bei jedem Release
> an und war ein Fehlalarm am Freeze-Tag. Verboten ist der *Pfad*, nicht das
> Wort.

Prüfliste:

- [ ] `sha256sum -c` sagt OK
- [ ] `wb.exe` liegt im ZIP
- [ ] `VERSION.txt` nennt den Tag, den du gepusht hast
- [ ] `./wb status` zeigt acht Übungen
- [ ] Der `find`-Befehl bleibt stumm — kein `trainer/`-Ordner, keine `LOESUNG.md`
- [ ] Auf der Release-Seite steht bei `-rc` **Pre-release**

Release-Seite:
<https://github.com/SebastianKrn/werkbank/releases>

---

## Wenn etwas schiefgeht

| Meldung | Ursache | Lösung |
|---|---|---|
| `Tag v… is not vMAJOR.MINOR.PATCH[-rcN]` | Tippfehler im Tag-Namen | Tag löschen, richtig neu setzen (siehe unten) |
| `Tag v… does not match runner/Cargo.toml version` | Version und Tag driften | `runner/Cargo.toml` anpassen, per PR mergen, dann neu taggen |
| `wb.exe missing from the ZIP` | Windows-Build ist durchgefallen | Log des `build (windows-latest)`-Jobs lesen |
| Stolperdraht (Exit 3) | `trainer/` oder etwas Lösungsförmiges ist ins Manifest geraten | `scripts/paket.sh` sagt in der Fehlermeldung, welche Datei |

**Einen Tag zurückziehen:**

```bash
git tag -d v0.1.0-rc3
git push origin :refs/tags/v0.1.0-rc3
gh release delete v0.1.0-rc3 --yes    # nur falls schon veröffentlicht
```

Danach neu taggen. Für Release-Kandidaten ist das unproblematisch. **Einen
veröffentlichten `v0.1.0` niemals überschreiben** — dann lieber `v0.1.1`.

---

## Lokales ZIP zum Ausprobieren

```bash
just package geraetetechnik --erlaube-ohne-windows
```

Baut ein **Linux-only** ZIP nach `dist/`. Ohne `wb.exe` — im Unterricht
wertlos. Deshalb muss der Verzicht ausdrücklich hingeschrieben werden, statt
als Warnung durchzurutschen.

---

## Der Freeze (M3b, Punkt 4)

Der Tag ist das ganze Freeze-Verfahren.

```bash
git switch main && git pull
git tag v0.1.0
git push origin v0.1.0
gh run watch
```

Danach:

- [ ] Ergebnis geprüft (Abschnitt „Nachher" oben)
- [ ] Release-Seite zeigt **kein** „Pre-release"
- [ ] `trainer/AUSTEILEN_A4.md` als A4 ausgedruckt, ein Blatt pro Lernendem
- [ ] Datum notiert: der Freeze muss **mindestens 3 Tage vor dem Pilottag**
      liegen (M3b Definition of Done)
