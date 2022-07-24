import type { RequestHandler } from '@sveltejs/kit';
import { list } from '$lib/posts';

export const GET: RequestHandler = async () => ({
	status: 200,
	body: await list()
});
