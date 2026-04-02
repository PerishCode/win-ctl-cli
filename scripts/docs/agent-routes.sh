#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

test -d "$REPO_DIR/app"
test -d "$REPO_DIR/docs"
test -d "$REPO_DIR/scripts/release"
test -d "$REPO_DIR/scripts/docs"
test -d "$REPO_DIR/scripts/manage"
printf 'PASS agent_routes\n'
