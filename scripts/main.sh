#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

usage() {
  cat <<USAGE
Usage: $(basename "$0") <brew|apt|rpm|dist>

Commands:
  brew   Generate a Homebrew formula (local file URL)
  apt    Build a .deb package (requires dpkg-deb, fakeroot)
  rpm    Build an .rpm package (requires rpmbuild)
  dist   Build release archives (tar.gz and zip)
  all    Build dist + brew + apt + rpm

Examples:
  scripts/main.sh dist
  scripts/main.sh brew
  scripts/main.sh apt
  scripts/main.sh rpm
USAGE
}

cmd=${1:-}
case "$cmd" in
  dist)
    # shellcheck source=./packages/common/lib.sh
    source "$SCRIPT_DIR/packages/common/lib.sh"
    make_dist_tarball >/dev/null
    make_dist_zip >/dev/null
    ;;
  all)
    # Build archives and all packages
    "$SCRIPT_DIR/main.sh" dist
    "$SCRIPT_DIR/main.sh" brew
    "$SCRIPT_DIR/main.sh" apt
    "$SCRIPT_DIR/main.sh" rpm
    ;;
  brew)
    "$SCRIPT_DIR/packages/brew/pack.sh"
    ;;
  apt)
    "$SCRIPT_DIR/packages/apt/pack.sh"
    ;;
  rpm)
    "$SCRIPT_DIR/packages/rpm/pack.sh"
    ;;
  ""|-h|--help|help)
    usage
    ;;
  *)
    echo "Unknown command: $cmd" >&2
    usage
    exit 1
    ;;
esac