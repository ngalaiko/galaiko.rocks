import { likesOf } from '$lib/webmentions';
import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async ({ url, params }) => {
	const href = new URL(`/${params.page}/`, url);
	return { status: 200, body: likesOf(href) as any[] };
};
