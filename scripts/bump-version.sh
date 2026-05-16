#!/usr/bin/env bash
# Usage: ./scripts/bump-version.sh 0.2.0
set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
    echo "Usage: $0 <version>  (e.g. 0.2.0)" >&2
    exit 1
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Validate semver format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: version must be in semver format (e.g. 1.2.3)" >&2
    exit 1
fi

echo "Bumping to $VERSION..."

# tauri.conf.json
jq --arg v "$VERSION" '.version = $v' "$ROOT/src-tauri/tauri.conf.json" \
    > "$ROOT/src-tauri/tauri.conf.json.tmp" \
    && mv "$ROOT/src-tauri/tauri.conf.json.tmp" "$ROOT/src-tauri/tauri.conf.json"

# package.json
jq --arg v "$VERSION" '.version = $v' "$ROOT/package.json" \
    > "$ROOT/package.json.tmp" \
    && mv "$ROOT/package.json.tmp" "$ROOT/package.json"

# Cargo.toml — sed is safe here since the version line is unambiguous
sed -i '' "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$VERSION\"/" \
    "$ROOT/src-tauri/Cargo.toml"

echo "Updated:"
echo "  src-tauri/tauri.conf.json -> $VERSION"
echo "  package.json              -> $VERSION"
echo "  src-tauri/Cargo.toml      -> $VERSION"
echo ""
echo "Next steps:"
echo "  git add src-tauri/tauri.conf.json package.json src-tauri/Cargo.toml"
echo "  git commit -m \"Bump version to $VERSION\""
echo "  git tag v$VERSION && git push origin main && git push origin v$VERSION"
