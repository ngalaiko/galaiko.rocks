import { compareDesc } from 'date-fns';

export type Post = {
  title: string;
  date: Date;
  default: any;
  path: string;
  aliases: string[];
  section: string;
  hidden: boolean;
  previous?: Post;
  next?: Post;
};

export const findByPathname = async (path: string) => {
  const posts = await list();
  const postsByAlias = posts.reduce((acc, post) => {
    post.aliases.forEach((alias) => {
      acc[alias] = post;
    });
    return acc;
  }, {});
  const postsByPath = posts.reduce((acc, post) => {
    acc[post.path] = post;
    return acc;
  }, {});
  return postsByPath[path] || postsByAlias[path];
};

const filenameToPath = (filename: string) => `/posts${filename.slice(1, -3)}/`;

export const list = () =>
  Promise.all(
    Object.entries(import.meta.glob('./**/*.md')).map(async ([filename, module]): Promise<Post> => {
      const m = await module();
      const { metadata } = m;
      return {
        ...metadata,
        default: m.default,
        path: filenameToPath(filename),
        aliases: metadata.aliases || [],
        tags: metadata.tags || [],
        section: metadata.section,
        date: new Date(metadata.date)
      };
    })
  ).then((posts) =>
    posts
      .filter((post) => !post.hidden)
      .sort((a, b) => compareDesc(a.date, b.date))
      .map((post, index, posts) => ({
        ...post,
        next: index > 0 ? posts[index - 1] : null,
        previous: index < posts.length - 1 ? posts[index + 1] : null
      }))
  );
