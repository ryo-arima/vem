#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common/lib.sh
source "$SCRIPT_DIR/../common/lib.sh"

main() {
  local root name version tarball sha dist formula_dir formula
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  dist="$(ensure_dist)"

  tarball="$(make_dist_tarball)"
  sha="$(sha256_file "$tarball")"

  formula_dir="$dist/brew"
  mkdir -p "$formula_dir"
  formula="$formula_dir/${name}.rb"

  cat > "$formula" <<'RUBY'
class __CLASS_NAME__ < Formula
  desc "VEM (Vim Environment Manager)"
  homepage "https://github.com/ryo-arima/vem"
  version "__VERSION__"

  on_macos do
    if Hardware::CPU.arm?
      url "__URL__"
      sha256 "__SHA__"
    else
      url "__URL__"
      sha256 "__SHA__"
    end
  end

  def install
    bin.install "__BINARY_NAME__"
  end
end
RUBY

  local class_name
  class_name="$(tr '[:lower:]-' '[:upper:]_' <<<"$name")"
  class_name="${class_name^}" # capitalize first

  # URL can be overridden via BREW_URL (e.g., GitHub Releases asset URL)
  local url
  if [[ -n "${BREW_URL:-}" ]]; then
    url="${BREW_URL}"
  else
    url="file://${tarball}"
  fi

  sed_inplace \
    -e "s/__CLASS_NAME__/${class_name}/" \
    -e "s/__VERSION__/${version}/" \
    -e "s|__URL__|${url}|g" \
    -e "s/__SHA__/${sha}/" \
    -e "s/__BINARY_NAME__/${name}/" \
    "$formula"

  echo "Generated Homebrew formula: $formula"
  if [[ -z "${BREW_URL:-}" ]]; then
    echo "Note: Replace file:// URL with a GitHub release URL before publishing, or set BREW_URL env."
  else
    echo "Using BREW_URL: ${BREW_URL}"
  fi

  # Copy helper installer script for local Homebrew installs
  cp -f "$SCRIPT_DIR/install-local.sh" "$formula_dir/install-local.sh"
  chmod +x "$formula_dir/install-local.sh"
  echo "Local Homebrew installer: $formula_dir/install-local.sh"
}

main "$@"
