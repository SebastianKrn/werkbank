#!/usr/bin/env bash
#
# Build the learner ZIP.
#
# This script is the ONLY place that knows how a learner ZIP is assembled.
# Both `just package` and the release workflow call it, so the classroom ZIP and
# the CI ZIP can never drift apart.
#
# Usage:
#   scripts/paket.sh <modul> <version> [options]
#
# Options:
#   --wb-linux PATH           Linux binary   (default: runner/target/release/wb)
#   --wb-windows PATH         Windows binary (default: dist/wb.exe)
#   --erlaube-ohne-windows    Build without wb.exe. Local development only —
#                             such a ZIP is useless in the classroom.
#
# Exit codes:
#   0  ok
#   1  usage error / unknown module
#   2  Windows binary missing and not explicitly waived
#   3  tripwire: a forbidden path or a symbolic link reached the package
#
# Note on names: the ZIP FILE carries the version, the FOLDER INSIDE does not.
# START_HIER.md tells learners to `cd C:\werkbank-geraetetechnik`, and that
# instruction must survive every release.
#
# A ZIP built with --erlaube-ohne-windows carries the waiver in its file name
# and in VERSION.txt. ADR 0006's reasoning applies: on freeze day a warning on
# stderr gets scrolled past, a file name does not.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# Paths that must never reach a learner: trainer material (ADR 0004) and
# anything solution-shaped (CLAUDE.md rule 6).
readonly VERBOTEN='trainer/|loesung|lösung'

# Stamped into the file name and into VERSION.txt of a waived build.
readonly TESTBAU_MARKE='TESTBAU-OHNE-WINDOWS'

# One fixed mtime for every staged file, so the same tag always produces the
# same SHA-256 (see the zip section below). Any date after 1980 does; the ZIP
# format cannot store anything earlier.
readonly ZEITSTEMPEL='202001010000.00'

usage() {
    echo "usage: scripts/paket.sh <modul> <version> [--wb-linux PATH] [--wb-windows PATH] [--erlaube-ohne-windows]" >&2
}

if [ $# -lt 2 ]; then
    usage
    exit 1
fi

modul="$1"
version="$2"
shift 2

wb_linux="runner/target/release/wb"
wb_windows="dist/wb.exe"
erlaube_ohne_windows="nein"

while [ $# -gt 0 ]; do
    case "$1" in
        --wb-linux)            wb_linux="${2:?--wb-linux needs a path}"; shift 2 ;;
        --wb-windows)          wb_windows="${2:?--wb-windows needs a path}"; shift 2 ;;
        --erlaube-ohne-windows) erlaube_ohne_windows="ja"; shift ;;
        *) echo "unknown option: $1" >&2; usage; exit 1 ;;
    esac
done

if [ ! -d "uebungen/${modul}" ]; then
    echo "No such module: uebungen/${modul} does not exist." >&2
    exit 1
fi

readonly ziel="dist/werkbank-${modul}"

# --- binaries ---------------------------------------------------------------

if [ ! -f "$wb_linux" ]; then
    echo "Linux binary missing: ${wb_linux}" >&2
    echo "Build it first: cargo build --release --manifest-path runner/Cargo.toml" >&2
    exit 1
fi

if [ -f "$wb_windows" ]; then
    mit_windows="ja"
else
    if [ "$erlaube_ohne_windows" = "nein" ]; then
        echo "FEHLER: ${wb_windows} fehlt — dieses ZIP hätte kein Windows-Binary." >&2
        echo "        Für den Pilotbetrieb ist es unbrauchbar (docs/MILESTONES.md, M3)." >&2
        echo "        Windows-Build kommt aus dem windows-latest-Job der Release-Pipeline." >&2
        echo "        Nur für lokale Tests: --erlaube-ohne-windows" >&2
        exit 2
    fi
    mit_windows="nein"
    echo "WARNUNG: ohne ${wb_windows} — dieses ZIP ist nur für lokale Tests." >&2
    echo "         Es heißt darum ...-${TESTBAU_MARKE}.zip und sagt das auch" >&2
    echo "         in VERSION.txt. Niemals als Release ausliefern." >&2
fi

# The waiver goes into the file name, not only into a warning: nobody must be
# able to hand this ZIP out believing it is the classroom build.
if [ "$mit_windows" = "ja" ]; then
    readonly zipdatei="dist/werkbank-${modul}-${version}.zip"
else
    readonly zipdatei="dist/werkbank-${modul}-${version}-${TESTBAU_MARKE}.zip"
fi

# --- assemble ---------------------------------------------------------------

rm -rf "$ziel"
mkdir -p "${ziel}/uebungen"

cp "$wb_linux" "${ziel}/wb"
chmod +x "${ziel}/wb"
if [ "$mit_windows" = "ja" ]; then
    cp "$wb_windows" "${ziel}/wb.exe"
fi

cp START_HIER.md "$ziel/"
cp -r "uebungen/${modul}/." "${ziel}/uebungen/"

# The content licence travels with the content (CC BY-NC-SA 4.0).
cp uebungen/LICENSE "${ziel}/uebungen/"

# The runner is MIT OR Apache-2.0 and statically links MIT/Apache dependencies,
# so its licences have to travel too. Own folder, .txt names, German folder
# name: a learner looks for exercises in `uebungen/` and can never mistake a
# licence for one.
mkdir -p "${ziel}/lizenzen"
cp LICENSE-MIT "${ziel}/lizenzen/wb-LIZENZ-MIT.txt"
cp LICENSE-APACHE "${ziel}/lizenzen/wb-LIZENZ-APACHE-2.0.txt"

# So a trainer can always answer "which build do you have?". The first line
# stays exactly `werkbank-<modul> <version>` — docs/TESTPROTOKOLL.md (B7) and
# docs/RELEASE.md read it that way.
echo "werkbank-${modul} ${version}" > "${ziel}/VERSION.txt"
if [ "$mit_windows" = "nein" ]; then
    echo "${TESTBAU_MARKE}: kein wb.exe enthalten." >> "${ziel}/VERSION.txt"
    echo "Nur für lokale Tests — nicht für den Unterricht." >> "${ziel}/VERSION.txt"
fi

# Dotfiles (.gitkeep and friends) never reach a learner.
find "$ziel" -name '.*' -not -name '.' -prune -exec rm -rf {} +

# --- manifest and tripwire --------------------------------------------------

mkdir -p dist
( cd dist && find "werkbank-${modul}" -type f | LC_ALL=C sort ) > dist/MANIFEST.txt

if grep -Eiq "$VERBOTEN" dist/MANIFEST.txt; then
    echo "FEHLER: verbotener Pfad im Paket — das darf niemals ausgeliefert werden:" >&2
    grep -Ei "$VERBOTEN" dist/MANIFEST.txt >&2
    exit 3
fi

# A symbolic link walks straight past the check above: the manifest lists only
# regular files, so a harmless-looking link into trainer/ never appears there —
# while zip would bake the content of its target into the archive. No links in
# the package, at all.
verknuepfungen="$(find "$ziel" -type l)"
if [ -n "$verknuepfungen" ]; then
    echo "FEHLER: Verknüpfung im Paket — das darf niemals ausgeliefert werden:" >&2
    printf '%s\n' "$verknuepfungen" >&2
    echo "        Eine Verknüpfung kann auf trainer/ oder auf eine Lösung zeigen." >&2
    exit 3
fi

# --- zip --------------------------------------------------------------------

# Reproducible archive: two builds of the same tag must have the same SHA-256,
# otherwise a checksum in a runbook proves nothing. A ZIP stores every file's
# mtime, and copying stamps each file with the time of the build — so the whole
# staged tree gets one fixed timestamp first. `-X` drops the remaining metadata
# (uid/gid, extra timestamp fields), and the sorted file list makes the entry
# order independent of the filesystem's directory order. `-y` stores symbolic
# links as links instead of copying what they point at — belt and braces next
# to the tripwire above.
find "$ziel" -exec touch -h -t "$ZEITSTEMPEL" -- {} +

# Permission bits land in the archive as well, and `cp` filters them through the
# build machine's umask. Normalise them, then give back the one bit that
# matters: `wb` must stay executable. wb.exe is for Windows, which has no such
# bit and does not care.
chmod -R u=rwX,go=rX "$ziel"
chmod 755 "${ziel}/wb"

rm -f "$zipdatei"
( cd dist && find "werkbank-${modul}" -print | LC_ALL=C sort \
    | zip -q -X -y -@ "$(basename "$zipdatei")" )

( cd dist && sha256sum "$(basename "$zipdatei")" ) > dist/SHA256SUMS.txt

echo "$zipdatei"
