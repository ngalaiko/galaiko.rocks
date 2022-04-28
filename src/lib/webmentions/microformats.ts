import type { Author, Reply, Webmention, Like, Mention, Repost } from './types';
import { mf2 } from 'microformats-parser';
import type { MicroformatRoot, Html } from 'microformats-parser/dist/types';

const allItems = (root: MicroformatRoot) =>
	root.children ? root.children.flatMap(allItems) : [root];

const not = (predicate: (i: MicroformatRoot) => boolean) => (i: MicroformatRoot) => !predicate(i);

const and =
	(...predicates: Array<(i: MicroformatRoot) => boolean>) =>
	(i: MicroformatRoot) =>
		predicates.every((predicate) => predicate(i));

const containsPropertyValue =
	(url: string) =>
	({ properties }: MicroformatRoot) =>
		Object.values(properties).some((values) => values.includes(url));

const mf2author = (root: MicroformatRoot): Author => {
	if (!root.type.includes('h-card')) {
		throw new Error('Not a h-card');
	}
	const picture = root.properties['photo']?.[0] as string;
	return {
		name: root.properties['name']?.[0] as string,
		url: root.properties['url']?.[0] as string,
		picture
	};
};

const isRepostOf =
	(url: string) =>
	({ properties }: MicroformatRoot) =>
		properties['repost-of']?.includes(url);

export const reposts = (webmention: Webmention): Repost[] => {
	const root = mf2(webmention.parsedSource.body, { baseUrl: webmention.sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isRepostOf(webmention.targetUrl)).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: webmention.sourceUrl },
			target: webmention.targetUrl,
			source: webmention.sourceUrl,
			timestamp: webmention.timestamp
		};
	});
};

const isLikeOf =
	(url: string) =>
	({ properties }: MicroformatRoot) =>
		properties['like-of']?.includes(url) || properties['like']?.includes(url);

export const likes = (webmention: Webmention): Like[] => {
	const targetUrl = webmention.targetUrl;
	const sourceContent = webmention.parsedSource;
	const root = mf2(sourceContent.body, { baseUrl: webmention.sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isLikeOf(targetUrl)).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: webmention.sourceUrl },
			target: targetUrl,
			source: webmention.sourceUrl,
			timestamp: webmention.timestamp
		};
	});
};

const isReplyTo =
	(url: string) =>
	({ properties, type }: MicroformatRoot) =>
		type.includes('h-entry') && properties['in-reply-to']?.includes(url);

export const replies = (webmention: Webmention): Reply[] => {
	const targetUrl = webmention.targetUrl;
	const sourceContent = webmention.parsedSource;
	const root = mf2(sourceContent.body, { baseUrl: webmention.sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isReplyTo(targetUrl)).map((root) => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		const content = root.properties['content']?.[0] as Html;
		const summary = root.properties['summary']?.[0] as string;
		const name = root.properties['name']?.[0] as string;
		const published = root.properties['published']?.[0] as string;
		const updated = root.properties['updated']?.[0] as string;
		return {
			target: targetUrl,
			source: webmention.sourceUrl,
			author: author ? mf2author(author) : { url: webmention.sourceUrl },
			content: content ? content.value : summary ? summary : name,
			published: published ? new Date(published).getTime() : webmention.timestamp,
			updated: updated ? new Date(updated).getTime() : undefined
		};
	});
};

const isMentionOf = (of: string) => (root: MicroformatRoot) =>
	and(containsPropertyValue(of), not(isReplyTo(of)), not(isLikeOf(of)), not(isRepostOf(of)))(root);

export const mentions = (webmention: Webmention): Mention[] => {
	const targetUrl = webmention.targetUrl;
	const sourceContent = webmention.parsedSource;
	const root = mf2(sourceContent.body, { baseUrl: webmention.sourceUrl });
	const items = root.items.flatMap(allItems);
	return items.filter(isMentionOf(targetUrl)).map((root): Mention => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: webmention.sourceUrl },
			target: targetUrl,
			source: webmention.sourceUrl,
			timestamp: webmention.timestamp
		};
	});
};
