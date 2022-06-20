import { compareDesc } from 'date-fns';

export type Post = {
	title: string;
	date: Date;
	path: string;
	html: string;
	aliases: string[];
	categories: string[];
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

export const list = () =>
	Promise.all(
		Object.entries(import.meta.glob('../../routes/posts/**/*.md')).map(
			async ([filename, module]): Promise<Post> => {
				const m = await module();
				const { metadata } = m;
				const path = filename.split('routes')[1].replace('.md', '/');
				return {
					...metadata,
					html: await m.default.render().html,
					path,
					aliases: metadata.aliases || [],
					categories: metadata.categories || [],
					date: new Date(metadata.date)
				};
			}
		)
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
