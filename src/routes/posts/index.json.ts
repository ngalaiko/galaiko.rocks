import type { RequestHandler } from '@sveltejs/kit';
import { list } from '$lib/posts';

export const get: RequestHandler = async () => {
	const posts = await list();
	return {
		status: 200,
		body: posts
	};
};
