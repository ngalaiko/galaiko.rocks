export type Post = {
	title: string;
	date: Date;
	path: string;
	html: string;
	aliases: string[];
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
		Object.entries(import.meta.glob('../../routes/posts/**/*.md')).map(
			async ([filename, module]): Promise<Post> => {
				const { metadata } = await module();
				const path = filename.split('routes')[1].replace('.md', '/');
				return {
					...metadata,
					path,
					aliases: metadata.aliases || [],
					date: new Date(metadata.date)
				};
			}
		)
	);
