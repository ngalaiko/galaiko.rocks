import jinja2
import os


def j2_environment(env):
    """Modify Jinja2 environment

    :param env: jinja2.environment.Environment
    :rtype: jinja2.environment.Environment
    """

    __dir__ = os.path.dirname(os.path.realpath(__file__))
    env.loader = jinja2.FileSystemLoader(os.path.join(__dir__, "templates"))


    # def join_path(self, template, parent):
    #     print(template)
    #     return os.path.join(os.path.dirname(parent), template)
    #
    # env.join_path = join_path

    return env
