import { compareDesc } from 'date-fns';

export type Comment = {
	pathname: string;
	created: Date;
	authorName: string;
	default: any;
};

export const findByPathname = async (pathname: string) =>
	list().then((comments) =>
		comments
			.filter((c) => c.pathname === pathname)
			.sort((a, b) => compareDesc(b.created, a.created))
	);

const filenameToPath = (filename: string) =>
	filename.split('/').slice(0, -1).join('/').slice(1) + '/';

export const list = () =>
	Promise.all(
		Object.entries(import.meta.glob('./**/*.md')).map(
			async ([filename, module]): Promise<Comment> => {
				const m = await module();
				const { metadata } = m;
				return {
					default: m.default,
					authorName: metadata.author_name,
					pathname: filenameToPath(filename),
					created: new Date(metadata.timestamp * 1000)
				};
			}
		)
	);
