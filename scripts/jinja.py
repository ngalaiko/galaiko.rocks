# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "jinja2",
#     "python-slugify",
# ]
# ///
import sys
import json
import jinja2
import hashlib
from urllib.parse import urlparse
from slugify import slugify
import os

root_dir = os.path.join(os.path.dirname(
    os.path.realpath(__file__)), os.path.pardir)


def main():
    if len(sys.argv) < 2:
        print("Usage: python jinja.py <path to template>")
        return

    # Read JSON object from stdin
    json_input = sys.stdin.read()
    data = json.loads(json_input)

    # Create a Jinja2 environment and render the template with the JSON data

    env = jinja2.Environment(
        loader=jinja2.FileSystemLoader(os.path.join(root_dir, "templates"))
    )

    # Add custom filters and tests
    env.filters.update(extra_filters())
    env.tests.update(extra_tests())

    tempalte_file = sys.argv[1]
    template = env.get_template(tempalte_file)
    result = template.render(data)

    # Print the result to stdout
    print(result)


def extra_filters():
    """Declare some custom filters."""

    def calculate_asset_md5hash(relpath):
        abspath = f"{root_dir}/assets{ relpath }"
        h = hashlib.md5()
        with open(abspath, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                h.update(chunk)
        return h.hexdigest()

    def static_with_hash(relpath):
        """Add version query parameter to the link with md5 hash of the file"""
        return f"{relpath}?v={calculate_asset_md5hash(relpath)}"

    return dict(
        slugify=slugify,
        urlparse=urlparse,
        static_with_hash=static_with_hash,
        split=lambda str, sep: str.split(sep),
        join=lambda ss, sep: sep.join(ss),
        skip=lambda ss, n: ss[n:],
    )


def extra_tests():
    """Declare some custom tests."""

    def is_url(s):
        try:
            result = urlparse(s)
            return all([result.scheme, result.netloc])
        except ValueError:
            return False

    return dict(url=is_url)


if __name__ == "__main__":
    main()
