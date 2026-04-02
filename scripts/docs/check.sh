#!/usr/bin/env bash
set -euo pipefail

bash "$(dirname "$0")/links.sh"
bash "$(dirname "$0")/alignment.sh"
bash "$(dirname "$0")/agent-meta.sh"
bash "$(dirname "$0")/agent-routes.sh"
