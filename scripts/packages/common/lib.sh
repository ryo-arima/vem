#!/usr/bin/env bash
set -euo pipefail

# Resolve project root (directory containing Cargo.toml)
project_root() {
  local dir start
  dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  start="$dir"
  # Climb up to 6 levels to find Cargo.toml to be robust to layout changes
  for _ in 1 2 3 4 5 6; do
    if [[ -f "$dir/Cargo.toml" ]]; then
      echo "$dir"
      return 0
    fi
    dir="$(dirname "$dir")"
  done
  echo "Error: Cargo.toml not found while ascending from $start" >&2
  exit 1
}

# Extract version from Cargo.toml [package]
project_version() {
  local root
  root="$(project_root)"
  awk -F '"' '/^[[:space:]]*version[[:space:]]*=[[:space:]]*"/ {print $2; exit}' "$root/Cargo.toml"
}

# Binary name (assumed to match package name)
project_name() {
  local root
  root="$(project_root)"
  awk -F '"' '/^[[:space:]]*name[[:space:]]*=[[:space:]]*"/ {print $2; exit}' "$root/Cargo.toml"
}

# Ensure release build exists; build if missing
ensure_release_build() {
  local root name bin target
  root="$(project_root)"
  name="$(project_name)"
  target="${TARGET:-}"
  if [[ -n "$target" ]]; then
    bin="$root/target/$target/release/$name"
    if [[ ! -x "$bin" ]]; then
      if command -v cross >/dev/null 2>&1; then
        (cd "$root" && cross build --release --target "$target")
      else
        (cd "$root" && cargo build --release --target "$target")
      fi
    fi
  else
    bin="$root/target/release/$name"
    if [[ ! -x "$bin" ]]; then
      (cd "$root" && cargo build --release)
    fi
  fi
  echo "$bin"
}

# Normalize architecture strings for different ecosystems
arch_deb() {
  local t="${TARGET:-}"
  local m
  if [[ -n "$t" ]]; then
    case "$t" in
      x86_64-*-linux-gnu*) m="x86_64" ;;
      aarch64-*-linux-gnu*) m="aarch64" ;;
      *) m="$(uname -m)" ;;
    esac
  else
    m="$(uname -m)"
  fi
  case "$m" in
    x86_64) echo "amd64" ;;
    arm64|aarch64) echo "arm64" ;;
    *) echo "$(uname -m)" ;;
  esac
}

arch_rpm() {
  local t="${TARGET:-}"
  local m
  if [[ -n "$t" ]]; then
    case "$t" in
      x86_64-*-linux-gnu*) m="x86_64" ;;
      aarch64-*-linux-gnu*) m="aarch64" ;;
      *) m="$(uname -m)" ;;
    esac
  else
    m="$(uname -m)"
  fi
  case "$m" in
    x86_64) echo "x86_64" ;;
    arm64|aarch64) echo "aarch64" ;;
    *) echo "$(uname -m)" ;;
  esac
}

# Generic arch label for filenames/tags
arch_generic() {
  local t="${TARGET:-}"
  if [[ -n "$t" ]]; then
    case "$t" in
      x86_64-*-linux-gnu*) echo "x86_64" ;;
      aarch64-*-linux-gnu*) echo "aarch64" ;;
      *) uname -m ;;
    esac
  else
    uname -m
  fi
}

# Create dist directory
ensure_dist() {
  local root
  root="$(project_root)"
  mkdir -p "$root/dist"
  echo "$root/dist"
}

# Create a tar.gz archive with the compiled binary and basic docs
make_dist_tarball() {
  local root name version arch out tmp bin
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  arch="$(arch_generic)"
  out="$(ensure_dist)/${name}-${version}-${arch}.tar.gz"
  tmp="$(mktemp -d)"
  bin="$(ensure_release_build)"
  mkdir -p "$tmp/$name"
  cp "$bin" "$tmp/$name/"
  [[ -f "$root/README.md" ]] && cp "$root/README.md" "$tmp/$name/" || true
  [[ -f "$root/LICENSE" ]] && cp "$root/LICENSE" "$tmp/$name/" || true
  (cd "$tmp" && tar -czf "$out" "$name")
  rm -rf "$tmp"
  echo "$out"
}

# Create a zip archive with the compiled binary and basic docs
make_dist_zip() {
  local root name version arch out tmp bin
  root="$(project_root)"
  name="$(project_name)"
  version="$(project_version)"
  arch="$(arch_generic)"
  out="$(ensure_dist)/${name}-${version}-${arch}.zip"
  tmp="$(mktemp -d)"
  bin="$(ensure_release_build)"
  mkdir -p "$tmp/$name"
  cp "$bin" "$tmp/$name/"
  [[ -f "$root/README.md" ]] && cp "$root/README.md" "$tmp/$name/" || true
  [[ -f "$root/LICENSE" ]] && cp "$root/LICENSE" "$tmp/$name/" || true
  (cd "$tmp" && zip -r -9 "$out" "$name" >/dev/null)
  rm -rf "$tmp"
  echo "$out"
}

# Compute sha256 of a file (portable)
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

# Portable in-place sed (handles GNU and BSD/macOS sed)
sed_inplace() {
  # Usage: sed_inplace -e 's/a/b/' -e 's/c/d/' file
  if sed --version >/dev/null 2>&1; then
    # GNU sed
    sed -i "$@"
  else
    # BSD/macOS sed requires a backup suffix (can be empty)
    sed -i '' "$@"
  fi
}
