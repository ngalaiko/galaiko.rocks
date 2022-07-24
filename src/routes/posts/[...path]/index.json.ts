import { findByPathname } from '$lib/posts';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ url }) => {
	const pathname = url.pathname.replace('.json', '/');
	const post = await findByPathname(pathname);
	if (!post) return { status: 404, body: 'Not found' };
	return { status: 200, body: post };
};
