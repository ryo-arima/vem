#!/usr/bin/env bash
set -euo pipefail

: "${GITHUB_TOKEN:?GITHUB_TOKEN is required}"
: "${VERSION:?VERSION is required}"
: "${DATE:?DATE is required}"

dist_dir="dist"

echo "Publishing latest release"
gh release delete latest -y --cleanup-tag || true
gh release create latest \
  --title "VEM latest (v${VERSION})" \
  --notes "Automated release from main.\nVersion: v${VERSION}" || true
echo "Uploading assets to latest..."
assets=( )
while IFS= read -r -d '' f; do assets+=("$f"); done < <(find "$dist_dir" -type f \( -name '*.tar.gz' -o -name '*.zip' -o -name '*.deb' -o -name '*.rpm' \) -print0)
if (( ${#assets[@]} > 0 )); then
  gh release upload latest "${assets[@]}" --clobber
else
  echo "No assets found under $dist_dir"
fi

mkdir -p ${dist_dir}/aliases
shopt -s nullglob
for f in ${dist_dir}/*; do
  base=$(basename "$f")
  case "$base" in
    *.tar.gz|*.zip|*.deb|*.rpm) : ;;
    *) continue ;;
  esac
  arch=""
  if [[ "$base" =~ (x86_64|amd64) ]]; then arch="x86_64"; fi
  if [[ "$base" =~ (aarch64|arm64) ]]; then arch="aarch64"; fi
  [[ -z "$arch" ]] && continue
  if [[ "$base" == *.tar.gz ]]; then ext=.tar.gz; elif [[ "$base" == *.zip ]]; then ext=.zip; elif [[ "$base" == *.deb ]]; then ext=.deb; elif [[ "$base" == *.rpm ]]; then ext=.rpm; else continue; fi
  alias="vem-linux-${arch}${ext}"
  cp -f "$f" "${dist_dir}/aliases/${alias}"
done
shopt -u nullglob
if compgen -G "${dist_dir}/aliases/*" >/dev/null; then
  gh release upload latest ${dist_dir}/aliases/* --clobber
fi

echo "Publishing date-tagged release v${VERSION}-${DATE}"
tag="v${VERSION}-${DATE}"
gh release create "$tag" \
  --title "VEM v${VERSION} - ${DATE}" \
  --notes "Automated release from main.\nVersion: v${VERSION}\nDate: ${DATE} (UTC)" || true
echo "Uploading assets to $tag..."
if (( ${#assets[@]} > 0 )); then
  gh release upload "$tag" "${assets[@]}" --clobber
fi
if compgen -G "${dist_dir}/aliases/*" >/dev/null; then
  gh release upload "$tag" ${dist_dir}/aliases/* --clobber
fi
