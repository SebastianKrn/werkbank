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
    cargo run --quiet --manifest-path {{runner}}/Cargo.toml -- \
        intern lint uebungen

# Everything the pipeline checks
ci: lint test lint-inhalt

# Turn accepted answers into expect_hash entries (authoring helper)
hash salt +antworten:
    cargo run --quiet --manifest-path {{runner}}/Cargo.toml -- \
        intern hash --salt {{salt}} {{antworten}}

# Build the learner ZIP (local testing).
#
# The assembly rules live in scripts/paket.sh — the one place that knows how a
# learner ZIP is built, so this recipe and the release pipeline cannot drift.
#
# The classroom needs wb.exe and this recipe cannot cross-compile it, so a local
# ZIP is refused unless you waive it:
#
#     just package geraetetechnik --erlaube-ohne-windows
#
# The real pilot ZIP comes from a tag via .github/workflows/release.yml (ADR 0006).
[unix]
package modul=default_modul *args:
    #!/usr/bin/env bash
    set -euo pipefail
    just release
    version="v$(grep -m1 '^version' {{runner}}/Cargo.toml | cut -d'"' -f2)"
    scripts/paket.sh {{modul}} "$version" {{args}}
