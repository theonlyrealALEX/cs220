#!/usr/bin/env bash

set -e
set -uo pipefail
IFS=$'\n\t'

# Imports library.
BASEDIR=$(dirname "$0")
source $BASEDIR/grade-utils.sh

RUNNERS=(
    "cargo"
    "cargo --release"
    "cargo_asan"
    "cargo_asan --release"
    "cargo_tsan"
    "cargo_tsan --release"
)

# Lints.
cargo fmt --check
cargo clippy

# Executes test for each runner.
for RUNNER in "${RUNNERS[@]}"; do
    echo "Running with $RUNNER..."

    TESTS=("--lib assignment04_grade")
    if [ $(run_tests) -ne 0 ]; then
        exit 1
    fi
done

exit 0
