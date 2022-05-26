import type { RequestHandler } from '@sveltejs/kit';
import { list } from '$lib/cocktails';

export const get: RequestHandler = async () => {
	const cocktails = await list();
	return {
		status: 200,
		body: cocktails
	};
};
