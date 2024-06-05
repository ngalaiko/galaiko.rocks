import jinja2
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
    return dict(slugify=slugify, urlparse=urlparse)


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
