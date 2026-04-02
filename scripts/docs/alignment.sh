#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

failures=0

pass() {
  printf 'PASS %s\n' "$1"
}

fail() {
  printf 'FAIL %s\n' "$1" >&2
  failures=$((failures + 1))
}

check_contains() {
  local label="$1"
  local file="$2"
  local pattern="$3"
  if grep -Eq "$pattern" "$file"; then
    pass "$label"
  else
    fail "$label"
  fi
}

check_contains "readme_links_zh_readme" "$REPO_DIR/README.md" 'README\.zh-CN\.md'
check_contains "readme_has_docs_site_link" "$REPO_DIR/README.md" 'https://win-ctl-cli\.pages\.dev/'
check_contains "readme_mentions_install" "$REPO_DIR/README.md" 'how-to/install'
check_contains "readme_mentions_use_profiles" "$REPO_DIR/README.md" 'how-to/use-profiles'
check_contains "readme_mentions_scoreboard" "$REPO_DIR/README.md" 'explanation/win-ctl-cli-score/native'
check_contains "readme_zh_mentions_install" "$REPO_DIR/README.zh-CN.md" 'how-to/install'
check_contains "readme_zh_mentions_use_profiles" "$REPO_DIR/README.zh-CN.md" 'how-to/use-profiles'
check_contains "readme_zh_mentions_scoreboard" "$REPO_DIR/README.zh-CN.md" 'explanation/win-ctl-cli-score/native'

if [ -f "$REPO_DIR/docs/index.md" ] && [ -f "$REPO_DIR/docs/zh-CN/index.md" ]; then
  pass "docs_root_mirror_present"
else
  fail "docs_root_mirror_present"
fi

if [ "$failures" -gt 0 ]; then
  exit 1
fi
