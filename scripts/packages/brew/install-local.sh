#!/usr/bin/env bash
set -euo pipefail

# Local Homebrew install using a local tarball
# Usage:
#   ./install-local.sh /path/to/vem-<version>-<arch>.tar.gz
#   # or zip
#   ./install-local.sh /path/to/vem-<version>-<arch>.zip

abs_path() {
  local p="$1"
  if command -v realpath >/dev/null 2>&1; then
    realpath "$p"
  else
    (cd "$(dirname "$p")" && pwd)/"$(basename "$p")"
  fi
}

sha256_file() {
  local file="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  else
    echo "Error: no sha256 tool found" >&2
    exit 1
  fi
}

usage() {
  echo "Usage: $(basename "$0") /path/to/vem-<version>-<arch>.tar.gz|.zip" >&2
}

main() {
  if [[ $# -ne 1 ]]; then
    usage; exit 1
  fi
  local tarball="$1"
  if [[ ! -f "$tarball" ]]; then
    echo "Error: file not found: $tarball" >&2
    exit 1
  fi

  local abspath sha base version name class tmp formula
  abspath="$(abs_path "$tarball")"
  sha="$(sha256_file "$abspath")"
  base="$(basename "$abspath")"
  name="vem"
  class="Vem"
  version="local"
  if [[ "$base" =~ ^${name}-([^-]+)- ]]; then
    version="${BASH_REMATCH[1]}"
  fi

  tmp="$(mktemp -d)"
  formula="$tmp/${name}.rb"
  cat > "$formula" <<RUBY
class ${class} < Formula
  desc "VEM (Vim Environment Manager)"
  homepage "https://github.com/ryo-arima/vem"
  version "${version}"

  url "file://${abspath}"
  sha256 "${sha}"

  def install
    bin.install "${name}"
  end
end
RUBY

  echo "Installing ${name} via Homebrew from local archive: ${abspath}"
  brew install --build-from-source "$formula"
  echo "Done. Try: ${name} --help"
}

main "$@"
