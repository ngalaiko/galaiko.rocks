import type { Post } from './post';

export const list = () =>
	Promise.all(
		Object.entries(import.meta.glob('../../routes/posts/**/*.md')).map(
			async ([filename, module]): Promise<Post> => {
				const { metadata } = await module();
				const path = filename.split('routes')[1].replace('.md', '/');
				return {
					...metadata,
					path,
					aliases: metadata.aliases || [],
					date: new Date(metadata.date)
				};
			}
		)
	);
