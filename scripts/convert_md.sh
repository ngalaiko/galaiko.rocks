#!/bin/sh

set -o errexit
set -o nounset
set -o pipefail

DIR="$(dirname $(readlink -f -- $0))"

frontmatter="$(cat "$1" | sed -n '/^---$/,/^---$/p' | sed '1d;$d')"
body="$(cat "$1" | sed -e '/^---$/,/^---$/d' | pandoc --lua-filter="$DIR/../filters/fix_images.lua" --lua-filter="$DIR/../filters/fix_links.lua")"

# TODO: this is kind of obscure. any simpler way to pass this path into the template?
echo "path: ${1/assets\/posts\//}"
echo "$frontmatter"
echo "content: |"
echo "$body" | while IFS= read -r line; do
    echo "  $line"
done 
