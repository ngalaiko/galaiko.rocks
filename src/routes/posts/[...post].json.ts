import { list } from '$lib/posts';

export const get = async ({ url }) => {
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

	const path = url.pathname.replace('.json', '/');
	const post = postsByPath[path] || postsByAlias[path];
	if (post) {
		return { status: 200, body: post };
	}
	return { status: 404 };
};
