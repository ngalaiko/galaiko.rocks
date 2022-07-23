import { findByPathname } from '$lib/posts';
import { likesOf, repliesTo, repostsOf } from '$lib/webmentions';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ url }) => {
	const pathname = url.pathname.replace('.json', '/');
	if (pathname.endsWith('/replies/')) {
		const postURL = new URL(pathname.replace('replies/', ''), url.origin);
		const post = await findByPathname(postURL.pathname);
		if (!post) return { status: 404, body: 'Not found' };
		return { status: 200, body: repliesTo(postURL) as any[] };
	} else if (pathname.endsWith('/likes/')) {
		const postURL = new URL(pathname.replace('likes/', ''), url.origin);
		const post = await findByPathname(postURL.pathname);
		if (!post) return { status: 404, body: 'Not found' };
		return { status: 200, body: likesOf(postURL) as any[] };
	} else if (pathname.endsWith('/reposts/')) {
		const postURL = new URL(pathname.replace('reposts/', ''), url.origin);
		const post = await findByPathname(postURL.pathname);
		if (!post) return { status: 404, body: 'Not found' };
		return { status: 200, body: repostsOf(postURL) as any[] };
	} else {
		const post = await findByPathname(pathname);
		if (!post) return { status: 404, body: 'Not found' };
		return { status: 200, body: post };
	}
};
