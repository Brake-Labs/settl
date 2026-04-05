#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "==> Generating doc pages from docs/"
node "$SCRIPT_DIR/generate-docs.mjs"

echo "==> Installing website dependencies"
cd "$ROOT_DIR/website"
npm install

echo "==> Building Astro site"
npm run build

echo "==> Done. Output in website/dist/"
