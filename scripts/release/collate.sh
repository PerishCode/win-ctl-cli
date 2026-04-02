#!/usr/bin/env bash
set -euo pipefail

root="${1:-dist}"

if [[ ! -d "$root" ]]; then
  echo "artifact root not found: $root" >&2
  exit 1
fi

tmp="$(mktemp)"
find "$root" -type f -name checksums.txt -print0 | while IFS= read -r -d '' file; do
  cat "$file" >> "$tmp"
done
sort -u "$tmp" > "$root/checksums.txt"
rm -f "$tmp"
