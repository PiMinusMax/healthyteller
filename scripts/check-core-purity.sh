#!/usr/bin/env bash
# crates/core knows neither UI nor network nor file system (AGENTS.md rule 9).
# Enforced as an allowlist of dependencies; extending it needs an ADR.
set -euo pipefail

allowed=(serde)

deps=$(cargo metadata --format-version 1 --no-deps |
  jq -r '.packages[] | select(.name == "healthyteller-core") | .dependencies[] | select(.kind == null) | .name')

failed=0
for dep in $deps; do
  if [[ ! " ${allowed[*]} " =~ " ${dep} " ]]; then
    echo "::error file=crates/core/Cargo.toml::crates/core must not depend on ${dep} (allowed: ${allowed[*]})"
    failed=1
  fi
done
if ((failed)); then
  exit 1
fi
echo "crates/core only depends on: ${deps:-nothing}"
