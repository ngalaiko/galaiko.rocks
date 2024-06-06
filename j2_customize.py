import jinja2
import hashlib
from urllib.parse import urlparse
from slugify import slugify
import os

__dir__ = os.path.dirname(os.path.realpath(__file__))


def j2_environment(env):
    """Modify Jinja2 environment

    :param env: jinja2.environment.Environment
    :rtype: jinja2.environment.Environment
    """

    # set loader to templates root
    env.loader = jinja2.FileSystemLoader(os.path.join(__dir__, "templates"))

    return env


def extra_filters():
    """Declare some custom filters.

    Returns: dict(name = function)
    """

    def calculate_asset_md5hash(relpath):
        abspath = f"{__dir__}/assets{ relpath }"
        h = hashlib.md5()
        b = bytearray(128 * 1024)
        mv = memoryview(b)
        with open(abspath, "rb", buffering=0) as f:
            while n := f.readinto(mv):
                h.update(mv[:n])
        return h.hexdigest()

    def static_with_hash(relpath):
        """Add version query parameter to the link with md5 hash of the file"""
        return f"{relpath}?v={calculate_asset_md5hash(relpath)}"

    return dict(
        slugify=slugify,
        urlparse=urlparse,
        static_with_hash=static_with_hash,
    )


def extra_tests():
    """Declare some custom tests

    Returns: dict(name = function)
    """

    def is_url(s):
        try:
            urlparse(s)
            return True
        except AttributeError:
            return False

    return dict(url=is_url)
