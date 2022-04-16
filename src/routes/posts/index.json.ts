import type { RequestHandler } from '@sveltejs/kit';
import { list } from '$lib/posts';
import { compareDesc } from 'date-fns';

export const get: RequestHandler = async () => {
	const posts = await list();
	return {
		status: 200,
		body: posts.sort((a, b) => compareDesc(new Date(a.date), new Date(b.date)))
	};
};
