import { repostsOf } from '$lib/webmentions/microformats';
import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async ({ url, params }) => {
	const href = new URL(`/${params.page}/`, url);
	return { status: 200, body: repostsOf(href) as any[] };
};
