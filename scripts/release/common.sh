#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
APP_DIR="$REPO_DIR/app"

fail_release() {
  printf 'FAIL release %s\n' "$1" >&2
  exit 1
}

normalize_version_tag() {
  local raw="$1"
  if [[ "$raw" == v* ]]; then
    printf '%s\n' "$raw"
  else
    printf 'v%s\n' "$raw"
  fi
}

parse_cargo_version() {
  awk '
    BEGIN { in_package=0 }
    /^\[package\]/ { in_package=1; next }
    /^\[/ && in_package { exit }
    in_package && $0 ~ /^version[[:space:]]*=/ {
      gsub(/"/, "", $0)
      sub(/^version[[:space:]]*=[[:space:]]*/, "", $0)
      print $0
      exit
    }
  ' "$APP_DIR/Cargo.toml"
}
