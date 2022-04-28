import type { Parsed, Author, Reply, Webmention, Like, Mention, Repost } from './types';
import { mf2 } from 'microformats-parser';
import type { MicroformatRoot, Html } from 'microformats-parser/dist/types';

const url = (i: URL | Parsed): URL => {
	if (i instanceof URL) return i;
	throw new Error('not url');
};

const parsed = (i: URL | Parsed): Parsed => {
	if (i instanceof URL) throw new Error('not parsed');
	return i;
};

const allItems = (root: MicroformatRoot) =>
	root.children ? root.children.flatMap(allItems) : [root];

const not = (predicate: (i: MicroformatRoot) => boolean) => (i: MicroformatRoot) => !predicate(i);

const and =
	(...predicates: Array<(i: MicroformatRoot) => boolean>) =>
	(i: MicroformatRoot) =>
		predicates.every((predicate) => predicate(i));

const containsPropertyValue =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		Object.values(properties).some((values) => values.includes(url.href));

const mf2author = (root: MicroformatRoot): Author => {
	if (!root.type.includes('h-card')) {
		throw new Error('Not a h-card');
	}
	const picture = root.properties['photo']?.[0] as string;
	return {
		name: root.properties['name']?.[0] as string,
		url: root.properties['url']?.[0] as string,
		picture: picture ? new URL(picture) : undefined
	};
};

const isRepostOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['repost-of']?.includes(url.href);

export const reposts = (webmention: Webmention): Repost[] => {
	const targetUrl = url(webmention.target);
	const sourceContent = parsed(webmention.source);
	const root = mf2(sourceContent.body, { baseUrl: sourceContent.url.href });
	const items = root.items.flatMap(allItems);
	return items.filter(isRepostOf(targetUrl)).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: sourceContent.url.href },
			target: targetUrl,
			source: sourceContent.url,
			timestamp: webmention.timestamp
		};
	});
};

const isLikeOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['like-of']?.includes(url.href) || properties['like']?.includes(url.href);

export const likes = (webmention: Webmention): Like[] => {
	const targetUrl = url(webmention.target);
	const sourceContent = parsed(webmention.source);
	const root = mf2(sourceContent.body, { baseUrl: sourceContent.url.href });
	const items = root.items.flatMap(allItems);
	return items.filter(isLikeOf(targetUrl)).map((root): Like => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: sourceContent.url.href },
			target: targetUrl,
			source: sourceContent.url,
			timestamp: webmention.timestamp
		};
	});
};

const isReplyTo =
	(url: URL) =>
	({ properties, type }: MicroformatRoot) =>
		type.includes('h-entry') && properties['in-reply-to']?.includes(url.href);

export const replies = (webmention: Webmention): Reply[] => {
	const targetUrl = url(webmention.target);
	const sourceContent = parsed(webmention.source);
	const root = mf2(sourceContent.body, { baseUrl: sourceContent.url.href });
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
			source: sourceContent.url,
			author: author ? mf2author(author) : { url: sourceContent.url.href },
			content: content ? content.value : summary ? summary : name,
			published: published ? new Date(published) : webmention.timestamp,
			updated: updated ? new Date(updated) : undefined
		};
	});
};

const isMentionOf = (of: URL) => (root: MicroformatRoot) =>
	and(containsPropertyValue(of), not(isReplyTo(of)), not(isLikeOf(of)), not(isRepostOf(of)))(root);

export const mentions = (webmention: Webmention): Mention[] => {
	const targetUrl = url(webmention.target);
	const sourceContent = parsed(webmention.source);
	const root = mf2(sourceContent.body, { baseUrl: sourceContent.url.href });
	const items = root.items.flatMap(allItems);
	return items.filter(isMentionOf(targetUrl)).map((root): Mention => {
		const author = root.properties['author']?.[0] as MicroformatRoot;
		return {
			author: author ? mf2author(author) : { url: sourceContent.url.href },
			target: targetUrl,
			source: sourceContent.url,
			timestamp: webmention.timestamp
		};
	});
};
