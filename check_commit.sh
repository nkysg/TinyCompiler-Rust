#!/bin/bash

# Provide a script for fast check the commit.

set -eo pipefail
# cargo fmt check
cargo fmt -- --check
# cargo clippy check
cargo clippy --all-targets -- -D warnings