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
  # Place compiled binary as Source0 for spec to install
  cp "$bin" "$topdir/SOURCES/$name"
  buildroot="$topdir/BUILDROOT"

  spec_template="$SCRIPT_DIR/spec.template.spec"
  spec="$specdir/${name}.spec"
  cp "$spec_template" "$spec"
  sed_inplace \
    -e "s/__NAME__/${name}/g" \
    -e "s/__VERSION__/${version}/g" \
    -e "s/__ARCH__/${arch}/g" \
    "$spec"

  # Determine rpmbuild target (maps to %{_target_cpu})
  local rb_target_cpu rb_target
  if [[ -n "${TARGET:-}" ]]; then
    case "$TARGET" in
      x86_64-*-linux-gnu*) rb_target_cpu="x86_64" ;;
      aarch64-*-linux-gnu*) rb_target_cpu="aarch64" ;;
      *) rb_target_cpu="$(uname -m)" ;;
    esac
  else
    rb_target_cpu="$(uname -m)"
  fi
  # Use a full target triple for better compatibility on Debian-based rpmbuilds
  case "$rb_target_cpu" in
    x86_64) rb_target="x86_64-linux" ;;
    aarch64|arm64) rb_target="aarch64-linux" ;;
    *) rb_target="${rb_target_cpu}-linux" ;;
  esac

  rpmbuild \
    --define "_topdir $topdir" \
    --define "_buildrootdir $buildroot" \
    --define "_target_cpu $rb_target_cpu" \
    --target "$rb_target" \
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
