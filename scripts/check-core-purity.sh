#!/usr/bin/env bash
# crates/core knows neither UI nor network nor file system (AGENTS.md rule 9).
# The simplest enforceable form: it has no dependencies at all.
set -euo pipefail

deps=$(cargo metadata --format-version 1 --no-deps |
  jq -r '.packages[] | select(.name == "healthyteller-core") | .dependencies[].name')

if [[ -n $deps ]]; then
  echo "::error file=crates/core/Cargo.toml::crates/core must not have dependencies, found: ${deps//$'\n'/, }"
  exit 1
fi
echo "crates/core has no dependencies."
