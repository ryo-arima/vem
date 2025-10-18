#!/usr/bin/env bash
set -euo pipefail

: "${GITHUB_TOKEN:?GITHUB_TOKEN is required}"
: "${TAG:?TAG is required}"
: "${ASSET_PATH:?ASSET_PATH is required}"
: "${ASSET_NAME:?ASSET_NAME is required}"

if [[ ! -f "$ASSET_PATH" ]]; then
  echo "Error: asset not found: $ASSET_PATH" >&2
  exit 1
fi

# Create release if missing (idempotent)
if ! gh release view "$TAG" >/dev/null 2>&1; then
  gh release create "$TAG" \
    --title "VEM $TAG" \
    --notes "Automated release for $TAG"
fi

gh release upload "$TAG" "$ASSET_PATH" --clobber --name "$ASSET_NAME"
