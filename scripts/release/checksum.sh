#!/usr/bin/env bash
set -euo pipefail

out="${1:-}"
shift || true

if [[ -z "$out" || $# -eq 0 ]]; then
  echo "usage: checksum.sh <output> <file> [file ...]" >&2
  exit 1
fi

mkdir -p "$(dirname "$out")"
: > "$out"

for file in "$@"; do
  if [[ ! -f "$file" ]]; then
    echo "missing file for checksum: $file" >&2
    exit 1
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" >> "$out"
  else
    shasum -a 256 "$file" >> "$out"
  fi
done
