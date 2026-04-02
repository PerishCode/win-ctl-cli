#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

grep -q '^# AGENTS' "$REPO_DIR/AGENTS.md"
grep -q '## Core Principle' "$REPO_DIR/AGENTS.md"
grep -q '## Directory Conventions' "$REPO_DIR/AGENTS.md"
printf 'PASS agent_meta\n'
