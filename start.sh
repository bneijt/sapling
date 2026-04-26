#!/usr/bin/bash
set -euo pipefail
mkdir -p tmp
exec cargo run -- --media-root tmp "$@"
