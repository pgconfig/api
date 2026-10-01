#!/usr/bin/env bash
# GoReleaser's Rust tool hook: place the binaries a matrix job built where the
# Rust builder expects Cargo's output. The publishing job never compiles.
set -euo pipefail

: "${PGCONFIG_PREBUILT_DIR:?Set PGCONFIG_PREBUILT_DIR to the downloaded build artifacts}"
target=""
for arg in "$@"; do
  case "$arg" in
    --target=*) target="${arg#--target=}" ;;
  esac
done

case "$target" in
  x86_64-unknown-linux-musl | aarch64-unknown-linux-musl) ;;
  x86_64-apple-darwin | aarch64-apple-darwin) ;;
  x86_64-pc-windows-msvc) ;;
  *)
    echo "Unsupported or missing release target: $target" >&2
    exit 1
    ;;
esac

source_dir="$PGCONFIG_PREBUILT_DIR/$target"
found=0
for binary in "$source_dir"/pgconfigctl* "$source_dir"/pgconfig-server*; do
  [[ -f "$binary" ]] || continue
  mkdir -p "target/$target/release"
  cp "$binary" "target/$target/release/"
  found=$((found + 1))
done

if [[ "$found" -ne 2 ]]; then
  echo "Expected pgconfigctl and pgconfig-server in $source_dir, found $found files" >&2
  exit 1
fi
