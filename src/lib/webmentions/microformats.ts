import { accepted } from './json';
import type { Parsed, Author, Reply } from './types';
import { mf2 } from 'microformats-parser';
import type { MicroformatRoot, Html } from 'microformats-parser/dist/types';

const href = (i: URL | Parsed): string => {
	if (i instanceof URL) return i.href;
	return i.url.href;
};

const allItems = (root: MicroformatRoot) =>
	root.children ? root.children.flatMap(allItems) : [root];

const convertToMicroformats = ({ contentType, body, url }: Parsed) =>
	contentType.includes('text/html') ? mf2(body, { baseUrl: url.href }).items.flatMap(allItems) : [];

const isReplyTo =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['in-reply-to']?.includes(url.href);

export const repliesTo = (url: URL) =>
	accepted
		.filter(({ target }) => href(target) === url.href)
		.map(({ source }) => source as Parsed)
		.map((source) => [source.url, convertToMicroformats(source)] as [URL, MicroformatRoot[]])
		.map(([sourceUrl, items]) => [sourceUrl, items.find(isReplyTo(url))] as [URL, MicroformatRoot])
		.filter(([, item]) => !!item)
		.map(([sourceURL, item]) => {
			try {
				return mf2reply(sourceURL, item);
			} catch (e) {
				return undefined;
			}
		})
		.filter((v) => !!v);

const isLikeOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['like-of']?.includes(url.href) || properties['like']?.includes(url.href);

export const likesOf = (url: URL) =>
	accepted
		.filter(({ target }) => href(target) === url.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(isLikeOf(url));

const isRepostOf =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['repost-of']?.includes(url.href);

export const repostsOf = (url: URL) =>
	accepted
		.filter(({ target }) => href(target) === url.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(isRepostOf(url));

const not = (predicate: (i: MicroformatRoot) => boolean) => (i: MicroformatRoot) => !predicate(i);

const and =
	(...predicates: Array<(i: MicroformatRoot) => boolean>) =>
	(i: MicroformatRoot) =>
		predicates.every((predicate) => predicate(i));

const containsPropertyValue =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		Object.values(properties).some((values) => values.includes(url.href));

export const mentionsOf = (url: URL) =>
	accepted
		.filter(({ target }) => href(target) === url.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(
			and(containsPropertyValue(url), not(isReplyTo(url)), not(isLikeOf(url)), not(isRepostOf(url)))
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

const mf2reply = (url: URL, root: MicroformatRoot): Reply => {
	if (!root.type.includes('h-entry')) {
		throw new Error('Not a h-entry');
	}

	const author = root.properties['author']?.[0] as MicroformatRoot;
	if (!author) {
		throw new Error(`No author`);
	}
	const content = root.properties['content']?.[0] as Html;
	const summary = root.properties['summary']?.[0] as string;
	const name = root.properties['name']?.[0] as string;
	const published = root.properties['published']?.[0] as string;
	const updated = root.properties['updated']?.[0] as string;
	return {
		url,
		author: mf2author(author),
		content: content ? content.value : summary ? summary : name,
		published: published ? new Date(published) : undefined,
		updated: updated ? new Date(updated) : undefined
	};
};
