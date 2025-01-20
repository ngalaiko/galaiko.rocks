#!/bin/sh

set -o errexit
set -o nounset
set -o pipefail

DIR="$(dirname $(readlink -f -- $0))"

function main() {
	local input_path="${1:-}"
	if [[ -z "${input_path}" ]]; then
		# If there's no argument, read from stdin
		input=$(cat)
		path="<stdin>"
	else
		# If there's an argument, read from file
		input=$(cat "${input_path}")
		path="${input_path#assets/posts/}"
	fi

    frontmatter="$(echo "$input" | sed -n '/^---$/,/^---$/p' | sed '1d;$d')"
	body="$(echo "$input" |\
		pandoc --lua-filter="$DIR/../filters/fix_links.lua")"

	echo "path: $path"
	echo "$(echo "$frontmatter" |  grep "title:")"
	echo "$(echo "$frontmatter" |  grep "date:")"
	echo "$(echo "$frontmatter" |  grep "id:")"
	echo "content: |2"
	echo "$body" | while IFS= read -r line; do
		echo "  $line"
	done
}

main "$@"
