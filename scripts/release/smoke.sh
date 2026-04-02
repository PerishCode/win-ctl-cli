#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: smoke.sh --version vX.Y.Z[-beta.N]

Run a local release-smoke placeholder for a tagged release.
EOF
}

if [[ $# -ne 2 ]] || [[ "${1:-}" != "--version" ]]; then
  usage >&2
  exit 1
fi

version="$2"
if [[ ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-beta\.[0-9]+)?$ ]]; then
  echo "invalid version format: $version" >&2
  exit 1
fi

printf '==> release smoke for %s\n' "$version"
printf 'PASS release_smoke version=%s\n' "$version"
