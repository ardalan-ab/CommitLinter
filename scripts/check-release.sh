#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

version="${1:-0.1.0}"
binary_names=(
  commit-linter-rs-linux-x64
  commit-linter-rs-linux-arm64
  commit-linter-rs-macos-x64
  commit-linter-rs-macos-arm64
  commit-linter-rs-windows-x64.exe
)

grep -q '"version": "'"$version"'"' packages/node/package.json
grep -q '^version = "'"$version"'"' packages/python/pyproject.toml
grep -q '^version: '"$version"'$' packages/flutter/pubspec.yaml
grep -q '^version = "'"$version"'"' crates/commit-linter-cli/Cargo.toml

if grep -R -n 'vendor/commit-linter-rs' README.md packages/php; then
  echo "error: placeholder Composer vendor remains" >&2
  exit 1
fi

for directory in \
  packages/node/vendor \
  packages/python/commit_linter_rs/vendor \
  packages/php/vendor-bin \
  packages/flutter/lib/vendor; do
  for binary in "${binary_names[@]}"; do
    test -f "$directory/$binary" || {
      echo "error: missing $directory/$binary" >&2
      exit 1
    }
  done
done

cargo test --workspace
npm_config_cache=/tmp/commit-linter-npm-cache npm pack ./packages/node --dry-run

echo "Release source checks passed for v$version"
