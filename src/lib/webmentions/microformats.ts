import type { Author, Reply, Like, Repost } from './types';
import { mf2 } from 'microformats-parser';
import type { MicroformatRoot, Html, Image } from 'microformats-parser/dist/types';

const allItems = (root: MicroformatRoot) =>
	root.children ? root.children.flatMap(allItems) : [root];

const mf2author = (root: MicroformatRoot): Author => {
	if (!root.type.includes('h-card')) {
		throw new Error('Not a h-card');
	}
	const pictureUrl = root.properties['photo']?.[0] as string;
	const pictureImage = root.properties['photo']?.[0] as Image;
	return {
		name: root.properties['name']?.[0] as string,
		url: root.properties['url']?.[0] as string,
		picture: pictureImage?.value ?? pictureUrl
	};
};

const isRepost = ({ properties }: MicroformatRoot) => properties['repost-of']?.length > 0;

export const reposts = (sourceUrl: string, html: string): Repost[] => {
	const root = mf2(html, { baseUrl: sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isRepost).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		const target = root.properties['repost-of']?.[0] as string;
		return {
			target,
			source: sourceUrl,
			author: author ? mf2author(author) : { url: sourceUrl },
			timestamp: undefined
		};
	});
};

const isLike = ({ properties }: MicroformatRoot) =>
	properties['like-of']?.length > 0 || properties['like']?.length > 0;

export const likes = (sourceUrl: string, html: string): Like[] => {
	const root = mf2(html, { baseUrl: sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isLike).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		const likeOf = root.properties['like-of']?.[0] as string;
		const like = root.properties['like']?.[0] as string;
		const target = like ?? likeOf;
		return {
			author: author ? mf2author(author) : { url: sourceUrl },
			target,
			source: sourceUrl,
			timestamp: undefined
		};
	});
};

const isReply = ({ properties }: MicroformatRoot) => properties['in-reply-to']?.length > 0;

export const replies = (sourceUrl: string, html: string): Reply[] => {
	const root = mf2(html, { baseUrl: sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isReply).map((root) => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		const contentHtml = root.properties['content']?.[0] as Html;
		const contentString = root.properties['content']?.[0] as string;
		const summary = root.properties['summary']?.[0] as string;
		const name = root.properties['name']?.[0] as string;
		const published = root.properties['published']?.[0] as string;
		const updated = root.properties['updated']?.[0] as string;
		const target = root.properties['in-reply-to']?.[0] as string;
		return {
			target,
			source: sourceUrl,
			author: author ? mf2author(author) : { url: sourceUrl },
			content: contentHtml?.value ?? contentString ?? summary ?? name,
			timestamp: published ? new Date(published).getTime() : undefined,
			updated: updated ? new Date(updated).getTime() : undefined
		};
	});
};

export const all = (sourceUrl: string, html: string): (Reply | Like | Repost)[] => [
	...replies(sourceUrl, html),
	...likes(sourceUrl, html),
	...reposts(sourceUrl, html)
];
