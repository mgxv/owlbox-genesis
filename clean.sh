#!/usr/bin/env bash

# Remove build artifacts, caches, and generated files.
# Usage: ./clean.sh [-n|--dry-run]

set -euo pipefail

cd "$(dirname "$0")"

DRY_RUN=0
case "${1:-}" in
    -n|--dry-run) DRY_RUN=1 ;;
    -h|--help) sed -n '2,4s/^# \?//p' "$0"; exit 0 ;;
    "") ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
esac

size() { du -sh "$1" 2>/dev/null | cut -f1 || echo "?"; }

# Top-level paths to wipe entirely.
PATHS=(
    src-tauri/target
    src-tauri/gen/schemas
    node_modules
    dist
    .vite
    .pnpm-store
    .eslintcache
)

# Glob patterns to remove anywhere in the tree (pruned dirs excluded).
NAMES=(
    '*.tsbuildinfo'
    '.DS_Store'
    'pnpm-debug.log*'
    'npm-debug.log*'
    'yarn-debug.log*'
    'vite.config.*.timestamp-*'
)

before=$(size .)
verb=$([[ $DRY_RUN -eq 1 ]] && echo "would rm" || echo "rm")

for p in "${PATHS[@]}"; do
    [[ -e $p ]] || continue
    printf '  %-9s %s (%s)\n' "$verb" "$p" "$(size "$p")"
    [[ $DRY_RUN -eq 0 ]] && rm -rf -- "$p"
done

find_expr=()
for n in "${NAMES[@]}"; do
    find_expr+=(-o -name "$n")
done

if [[ $DRY_RUN -eq 1 ]]; then
    find . -path ./node_modules -prune -o -path ./src-tauri/target -prune -o \
        \( "${find_expr[@]:1}" \) -print 2>/dev/null \
        | awk -v v="$verb" '{ printf "  %-9s %s\n", v, $0 }'
else
    find . -path ./node_modules -prune -o -path ./src-tauri/target -prune -o \
        \( "${find_expr[@]:1}" \) -delete 2>/dev/null || true
fi

after=$(size .)
echo
if [[ $DRY_RUN -eq 1 ]]; then
    echo "Dry run. Current size: $before"
else
    echo "Done. $before → $after"
fi
