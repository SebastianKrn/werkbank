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
#   3  tripwire: a forbidden path reached the package
#
# Note on names: the ZIP FILE carries the version, the FOLDER INSIDE does not.
# START_HIER.md tells learners to `cd C:\werkbank-geraetetechnik`, and that
# instruction must survive every release.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# Paths that must never reach a learner: trainer material (ADR 0004) and
# anything solution-shaped (CLAUDE.md rule 6).
readonly VERBOTEN='trainer/|loesung|lösung'

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
readonly zipdatei="dist/werkbank-${modul}-${version}.zip"

# --- binaries ---------------------------------------------------------------

if [ ! -f "$wb_linux" ]; then
    echo "Linux binary missing: ${wb_linux}" >&2
    echo "Build it first: cargo build --release --manifest-path runner/Cargo.toml" >&2
    exit 1
fi

if [ ! -f "$wb_windows" ]; then
    if [ "$erlaube_ohne_windows" = "nein" ]; then
        echo "FEHLER: ${wb_windows} fehlt — dieses ZIP hätte kein Windows-Binary." >&2
        echo "        Für den Pilotbetrieb ist es unbrauchbar (docs/MILESTONES.md, M3)." >&2
        echo "        Windows-Build kommt aus dem windows-latest-Job der Release-Pipeline." >&2
        echo "        Nur für lokale Tests: --erlaube-ohne-windows" >&2
        exit 2
    fi
    echo "WARNUNG: ohne ${wb_windows} — dieses ZIP ist nur für lokale Tests." >&2
fi

# --- assemble ---------------------------------------------------------------

rm -rf "$ziel"
mkdir -p "${ziel}/uebungen"

cp "$wb_linux" "${ziel}/wb"
chmod +x "${ziel}/wb"
if [ -f "$wb_windows" ]; then
    cp "$wb_windows" "${ziel}/wb.exe"
fi

cp START_HIER.md "$ziel/"
cp -r "uebungen/${modul}/." "${ziel}/uebungen/"

# The content licence travels with the content (CC BY-NC-SA 4.0).
cp uebungen/LICENSE "${ziel}/uebungen/"

# So a trainer can always answer "which build do you have?".
echo "werkbank-${modul} ${version}" > "${ziel}/VERSION.txt"

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

# --- zip --------------------------------------------------------------------

rm -f "$zipdatei"
( cd dist && zip -r -q -X "$(basename "$zipdatei")" "werkbank-${modul}" )

( cd dist && sha256sum "$(basename "$zipdatei")" ) > dist/SHA256SUMS.txt

echo "$zipdatei"
