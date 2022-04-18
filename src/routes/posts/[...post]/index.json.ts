import { list } from '$lib/posts';
import { likesOf, mentionsOf, repliesTo, repostsOf } from '$lib/webmentions/microformats';
import type { RequestHandler } from '@sveltejs/kit';

const findByAlias = async (path: string) => {
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

export const get: RequestHandler = async ({ url }) => {
	const path = url.pathname.replace('.json', '/');
	if (path.endsWith('/replies/')) {
		const postURL = new URL(path.replace('replies/', ''), url.origin);
		return { status: 200, body: repliesTo(postURL) as any[] };
	} else if (path.endsWith('/likes/')) {
		const postURL = new URL(path.replace('likes/', ''), url.origin);
		return { status: 200, body: likesOf(postURL) as any[] };
	} else if (path.endsWith('/mentions/')) {
		const postURL = new URL(path.replace('mentions/', ''), url.origin);
		return { status: 200, body: mentionsOf(postURL) as any[] };
	} else if (path.endsWith('/reposts/')) {
		const postURL = new URL(path.replace('reposts/', ''), url.origin);
		return { status: 200, body: repostsOf(postURL) as any[] };
	} else {
		const post = await findByAlias(path);
		return post ? { status: 301, redirect: post.path } : { status: 404, body: 'Not Found' };
	}
};
