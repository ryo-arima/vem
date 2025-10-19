#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common/lib.sh
source "$SCRIPT_DIR/../common/lib.sh"

require_tools() {
  for t in dpkg-deb fakeroot; do
    command -v "$t" >/dev/null 2>&1 || { echo "Error: $t is required" >&2; exit 1; }
  done
}

main() {
  require_tools
  local root name version tag arch dist bin pkgdir ctlfile deb maintainer tmpl
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  tag="$(project_tag)"
  arch="$(arch_deb)"
  dist="$(ensure_dist)"
  bin="$(ensure_release_build)"
  maintainer="${DEBFULLNAME:-$USER} <${EMAIL:-$USER@localhost}>"

  pkgdir="$(mktemp -d)"
  mkdir -p "$pkgdir/DEBIAN" "$pkgdir/usr/local/bin"
  install -m 0755 "$bin" "$pkgdir/usr/local/bin/$name"

  tmpl="$SCRIPT_DIR/control.template"
  ctlfile="$pkgdir/DEBIAN/control"
  cp "$tmpl" "$ctlfile"
  sed_inplace \
    -e "s/__NAME__/${name}/g" \
    -e "s/__VERSION__/${version}/g" \
    -e "s/__ARCH__/${arch}/g" \
    -e "s/__MAINTAINER__/${maintainer//\//\/}/g" \
    "$ctlfile"
  # Ensure control file ends with a newline (dpkg-deb requires it)
  tail -c1 "$ctlfile" | read -r _ || echo >> "$ctlfile"

  deb="$dist/${name}_${tag}_${arch}.deb"
  fakeroot dpkg-deb --build "$pkgdir" "$deb"
  rm -rf "$pkgdir"
  echo "Built deb package: $deb"
}

main "$@"
