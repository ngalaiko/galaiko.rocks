import { accepted } from './json';
import type { Parsed, Author, Reply, Webmention } from './types';
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

const convertToMicroformats = ({ contentType, body, url }: Parsed) =>
	contentType.includes('text/html') ? mf2(body, { baseUrl: url.href }).items.flatMap(allItems) : [];

const isReplyTo =
	(url: URL) =>
	({ properties, type }: MicroformatRoot) =>
		type.includes('h-entry') && properties['in-reply-to']?.includes(url.href);

export const repliesTo = (to: URL) =>
	accepted.flatMap(replies).filter((reply) => reply.to.href === to.href);

const isLikeOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['like-of']?.includes(url.href) || properties['like']?.includes(url.href);

export const likesOf = (of: URL) =>
	accepted
		.filter(({ target }) => url(target).href === of.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(isLikeOf(of));

const isRepostOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['repost-of']?.includes(url.href);

export const repostsOf = (of: URL) =>
	accepted
		.filter(({ target }) => url(target).href === of.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(isRepostOf(of));

const not = (predicate: (i: MicroformatRoot) => boolean) => (i: MicroformatRoot) => !predicate(i);

const and =
	(...predicates: Array<(i: MicroformatRoot) => boolean>) =>
	(i: MicroformatRoot) =>
		predicates.every((predicate) => predicate(i));

const containsPropertyValue =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		Object.values(properties).some((values) => values.includes(url.href));

export const mentionsOf = (of: URL) =>
	accepted
		.filter(({ target }) => url(target).href === of.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(
			and(containsPropertyValue(of), not(isReplyTo(of)), not(isLikeOf(of)), not(isRepostOf(of)))
		);

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

const replies = (webmention: Webmention): Reply[] => {
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
			to: targetUrl,
			url: sourceContent.url,
			author: author ? mf2author(author) : { url: sourceContent.url.href },
			content: content ? content.value : summary ? summary : name,
			published: published ? new Date(published) : webmention.timestamp,
			updated: updated ? new Date(updated) : undefined
		};
	});
};
