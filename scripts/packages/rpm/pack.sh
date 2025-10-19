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
  local root name version tag arch dist bin topdir specdir buildroot spec rpm spec_template release
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  tag="$(project_tag)"
  arch="$(arch_rpm)"
  dist="$(ensure_dist)"
  bin="$(ensure_release_build)"
  # Extract datetime from tag (format: version-datetime)
  release="${tag#*-}"
  # If tag doesn't contain '-', use '1' as release
  if [[ "$release" == "$tag" ]]; then
    release="1"
  fi

  topdir="$(mktemp -d)"
  mkdir -p "$topdir/BUILD" "$topdir/RPMS" "$topdir/SOURCES" "$topdir/SPECS" "$topdir/SRPMS"
  specdir="$topdir/SPECS"
  # Place compiled binary as Source0 for spec to install
  cp "$bin" "$topdir/SOURCES/$name"
  buildroot="$topdir/BUILDROOT"

  spec_template="$SCRIPT_DIR/spec.template.spec"
  spec="$specdir/${name}.spec"
  cp "$spec_template" "$spec"
  # Use version for Version field, datetime for Release field
  sed_inplace \
    -e "s/__NAME__/${name}/g" \
    -e "s/__VERSION__/${version}/g" \
    -e "s/__RELEASE__/${release}/g" \
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
  # Normalize host cpu
  local host_cpu
  case "$(uname -m)" in
    x86_64) host_cpu="x86_64" ;;
    aarch64|arm64) host_cpu="aarch64" ;;
    *) host_cpu="$(uname -m)" ;;
  esac
  # If attempting to build RPM for a different CPU than host, skip gracefully.
  if [[ "$rb_target_cpu" != "$host_cpu" ]]; then
    echo "Skipping RPM packaging for target CPU '$rb_target_cpu' on host '$host_cpu' (cross-rpmbuild not supported in this workflow)."
    exit 0
  fi
  # Use distro-expected triples for Debian/Ubuntu rpmbuild
  case "$rb_target_cpu" in
    x86_64) rb_target="x86_64-linux-gnu" ;;
    aarch64|arm64) rb_target="aarch64-linux-gnu" ;;
    *) rb_target="${rb_target_cpu}-linux-gnu" ;;
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
