# Werkbank build tasks.
#
# CI does not use `just` — it runs the same cargo commands directly, so that a
# missing tool can never be the reason a pipeline is red. Keep both in sync.

runner := "runner"
default_modul := "geraetetechnik"

# Show all recipes
default:
    @just --list

# Build the runner (debug)
build:
    cargo build --manifest-path {{runner}}/Cargo.toml

# Build the portable release binary
release:
    cargo build --release --manifest-path {{runner}}/Cargo.toml

# Format the code
fmt:
    cargo fmt --manifest-path {{runner}}/Cargo.toml

# Formatting + clippy, warnings are errors
lint:
    cargo fmt --manifest-path {{runner}}/Cargo.toml --check
    cargo clippy --manifest-path {{runner}}/Cargo.toml --all-targets -- -D warnings

# Unit and integration tests
test:
    cargo test --manifest-path {{runner}}/Cargo.toml

# Validate exercise content — content is code, a broken exercise.toml fails
lint-inhalt:
    cargo run --quiet --manifest-path {{runner}}/Cargo.toml -- \
        intern lint {{runner}}/tests/fixtures/modul-demo/uebungen
    @# M2: add the real module here once uebungen/ exists:
    @#   cargo run --quiet --manifest-path {{runner}}/Cargo.toml -- intern lint uebungen

# Everything the pipeline checks
ci: lint test lint-inhalt

# Turn accepted answers into expect_hash entries (authoring helper)
hash salt +antworten:
    cargo run --quiet --manifest-path {{runner}}/Cargo.toml -- \
        intern hash --salt {{salt}} {{antworten}}

# Build the learner ZIP. Content arrives in M2 — until then this stops early
# with a clear message instead of shipping an empty package.
[unix]
package modul=default_modul:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d "uebungen/{{modul}}" ]; then
        echo "No content yet: uebungen/{{modul}} does not exist."
        echo "Exercises are milestone M2 (docs/MILESTONES.md). Nothing to package."
        exit 1
    fi
    just release
    rm -rf dist/werkbank-{{modul}}
    mkdir -p dist/werkbank-{{modul}}/uebungen
    cp {{runner}}/target/release/wb dist/werkbank-{{modul}}/
    cp START_HIER.md dist/werkbank-{{modul}}/
    cp -r uebungen/{{modul}}/. dist/werkbank-{{modul}}/uebungen/
    # trainer/ and dotfiles never reach a learner ZIP (SPEC §5)
    find dist/werkbank-{{modul}} -name '.*' -not -name '.' -prune -exec rm -rf {} +
    (cd dist && find werkbank-{{modul}} -type f | sort > MANIFEST.txt)
    (cd dist && zip -r -q werkbank-{{modul}}.zip werkbank-{{modul}})
    echo "dist/werkbank-{{modul}}.zip"
