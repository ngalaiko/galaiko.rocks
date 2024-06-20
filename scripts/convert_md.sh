#!/bin/sh

set -o errexit
set -o nounset
set -o pipefail

DIR="$(dirname $(readlink -f -- $0))"

if [ -z "${1:-}" ]; then
    # If there's no argument, read from stdin
    input=$(cat)
    frontmatter="$(echo "$input" | sed -n '/^---$/,/^---$/p' | sed '1d;$d')"
    body="$(echo "$input" | sed -e '/^---$/,/^---$/d' | pandoc --lua-filter="$DIR/../filters/fix_md.lua")"
    path="<stdin>"
else
    # If there's an argument, read from file
    frontmatter="$(cat "$1" | sed -n '/^---$/,/^---$/p' | sed '1d;$d')"
    body="$(cat "$1" | sed -e '/^---$/,/^---$/d' | pandoc --lua-filter="$DIR/../filters/fix_md.lua")"
    path="${1/assets\/posts\//}"
fi

# Output the result
echo "path: $path"
echo "$frontmatter"
echo "content: |"
echo "$body" | while IFS= read -r line; do
    echo "  $line"
done

