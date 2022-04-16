import type { Post } from '$lib/types/post';

export const get = async ({ url }) => {
	const modules = Object.entries(import.meta.glob('../../routes/posts/**/*.md'));
	const posts = await Promise.all(
		modules.map(async ([filename, module]): Promise<Post> => {
			const { metadata } = await module();
			const path = filename.split('routes')[1].replace('.md', '/');
			return {
				...metadata,
				path,
				aliases: metadata.aliases || []
			};
		})
	);

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

	const path = url.pathname.replace('.json', '/');
	const post = postsByPath[path] || postsByAlias[path];
	if (post) {
		return { status: 200, body: post };
	}
	return { status: 404 };
};
