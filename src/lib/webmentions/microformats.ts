import { accepted } from './json';
import type { Parsed } from './types';
import { mf2 } from 'microformats-parser';
import type { MicroformatRoot } from 'microformats-parser/dist/types';

const href = (i: URL | Parsed): string => {
	if (i instanceof URL) return i.href;
	return i.url.href;
};

const convertToMicroformats = ({ contentType, body, url }: Parsed) =>
	contentType.includes('text/html')
		? mf2(body, { baseUrl: url.origin }).items.flatMap((item) =>
				item.children ? item.children.flat() : [item]
		  )
		: [];

const isReplyTo =
	(url: URL) =>
	({ properties }: MicroformatRoot) =>
		properties['in-reply-to']?.includes(url.href);

export const repliesTo = (url: URL) =>
	accepted
		.filter(({ target }) => href(target) === url.href)
		.map(({ source }) => source as Parsed)
		.flatMap(convertToMicroformats)
		.filter(isReplyTo(url));

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
