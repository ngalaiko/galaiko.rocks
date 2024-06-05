import jinja2
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
