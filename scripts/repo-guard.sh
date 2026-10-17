#!/usr/bin/env bash
# Fails if external data ends up in the repository (AGENTS.md rule 1): anything
# under data/, database or archive files, or any file above the size limit.
#
# Usage: repo-guard.sh [revision-range]
# Always checks the tree at HEAD. With a range, also checks every file added or
# changed by each commit in it, so data that was committed and deleted again
# is caught as well.
set -euo pipefail

max_bytes=$((1024 * 1024))
forbidden_ext='\.(sqlite|sqlite3|db|zip|gz|tgz|bz2|xz|7z|zst|parquet|jsonl)$'
failed=0

report() {
  echo "::error file=$2::$1 ($3)"
  failed=1
}

check() {
  local rev=$1 path=$2 size
  if [[ $path == data/* ]]; then
    report "files under data/ are generated locally and must not be committed" "$path" "$rev"
  fi
  if [[ $path =~ $forbidden_ext ]]; then
    report "database and archive files must not be committed" "$path" "$rev"
  fi
  size=$(git cat-file -s "$rev:$path")
  if ((size > max_bytes)); then
    report "file is larger than $max_bytes bytes; external data belongs on the device, not in the repo" "$path" "$rev"
  fi
}

while IFS= read -r -d '' path; do
  check HEAD "$path"
done < <(git ls-tree -r -z --name-only HEAD)

if [[ -n ${1:-} ]]; then
  # Separate assignment so that an invalid range fails the script.
  revs=$(git rev-list "$1")
  for rev in $revs; do
    while IFS= read -r -d '' path; do
      check "$rev" "$path"
    done < <(git diff-tree -r -z --no-commit-id --name-only --diff-filter=AMR --root "$rev")
  done
fi

if ((failed)); then
  exit 1
fi
echo "Repo guard: no external data found."
