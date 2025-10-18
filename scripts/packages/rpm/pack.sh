#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common/lib.sh
source "$SCRIPT_DIR/../common/lib.sh"

require_tools() {
  for t in rpmbuild; do
    command -v "$t" >/dev/null 2>&1 || { echo "Error: $t is required" >&2; exit 1; }
  done
}

main() {
  require_tools
  local root name version arch dist bin topdir specdir buildroot spec rpm spec_template
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  arch="$(arch_rpm)"
  dist="$(ensure_dist)"
  bin="$(ensure_release_build)"
  # Try to get homepage/license from Cargo.toml (best effort)

  topdir="$(mktemp -d)"
  mkdir -p "$topdir/BUILD" "$topdir/RPMS" "$topdir/SOURCES" "$topdir/SPECS" "$topdir/SRPMS"
  specdir="$topdir/SPECS"
  buildroot="$topdir/BUILDROOT/${name}-${version}-1.${arch}"
  mkdir -p "$buildroot/usr/local/bin"
  install -m 0755 "$bin" "$buildroot/usr/local/bin/$name"

  spec_template="$SCRIPT_DIR/spec.template.spec"
  spec="$specdir/${name}.spec"
  cp "$spec_template" "$spec"
  sed_inplace \
    -e "s/__NAME__/${name}/g" \
    -e "s/__VERSION__/${version}/g" \
    -e "s/__ARCH__/${arch}/g" \
    "$spec"

  rpmbuild \
    --define "_topdir $topdir" \
    --define "_buildrootdir $buildroot" \
    -bb "$spec"

  rpm=$(find "$topdir/RPMS" -name "*.rpm" -print -quit)
  mkdir -p "$dist"
  if [[ -n "$rpm" ]]; then
    cp "$rpm" "$dist/"
    echo "Built rpm package: $dist/$(basename "$rpm")"
  else
    echo "Error: RPM not produced" >&2
    exit 1
  fi

  rm -rf "$topdir"
}

main "$@"
