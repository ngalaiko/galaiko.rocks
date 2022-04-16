import type { RequestHandler } from '@sveltejs/kit';
import type { Post } from '$lib/types/post';
import { compareDesc } from 'date-fns';

export const get: RequestHandler = async () => {
	const modules = Object.entries(import.meta.glob('../../routes/posts/**/*.md'));
	const posts = await Promise.all(
		modules.map(async ([filename, module]): Promise<Post> => {
			const { metadata } = await module();
			const path = filename.split('routes')[1].replace('.md', '/');
			return {
				...metadata,
				path,
				aliases: metadata.aliases || []
			};
		})
	);
	return {
		status: 200,
		body: posts.sort((a, b) => compareDesc(new Date(a.date), new Date(b.date)))
	};
};
