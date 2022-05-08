export type Post = {
	title: string;
	date: Date;
	path: string;
	html: string;
	aliases: string[];
	categories: string[];
	hidden: boolean;
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
		Object.entries(import.meta.glob('../../posts/**/*.md')).map(
			async ([filename, module]): Promise<Post> => {
				const m = await module();
				const { metadata } = m;
				return {
					...metadata,
					html: await m.default.render().html,
					path: `/posts${filename.split('posts')[1].replace('.md', '/')}`,
					aliases: metadata.aliases || [],
					categories: metadata.categories || [],
					date: new Date(metadata.date)
				};
			}
		)
	);
