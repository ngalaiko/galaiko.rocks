import { list } from '$lib/posts';
import { nanoid } from 'nanoid';
import { provideDB } from './database';
import { accepted } from './json';
import { likes, mentions, replies, reposts } from './microformats';
import { Status, type Webmention } from './types';

const validDomains = {
	'galaiko.rocks': true
};

if (import.meta.env.DEV) validDomains['localhost'] = true;

const validateDomain = (url: URL) => {
	if (!validDomains[url.hostname]) {
		throw new ValidationError(`unsupported domain: "${url.hostname}"`);
	}
};

const supportedProtocols = {
	'http:': true,
	'https:': true
};

const validateProtocol = (url: URL) => {
	if (!supportedProtocols[url.protocol]) {
		throw new Error(`unsupported protocol: "${url.protocol}"`);
	}
};

const parseURL = (value: string): URL => {
	try {
		return new URL(value);
	} catch {
		throw new Error('must be valid url');
	}
};

const validateURL = (url?: string): URL => {
	if (!url || !url.length) {
		throw new Error('must not be empty');
	}
	const parsed = parseURL(url);
	validateProtocol(parsed);
	return parsed;
};

export class ValidationError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'ValidationError';
	}
}

const validatePathname = async (pathname: string) => {
	const validPathnames = { '/': true, '/records/': true, '/restaurants_and_cafes/': true };
	await list().then((posts) =>
		posts.map(({ path }) => path).forEach((path) => (validPathnames[path] = true))
	);
	if (!validPathnames[pathname]) throw new ValidationError(`"${pathname}" can't be webmentioned`);
};

const validateTarget = async (target: URL) => {
	validateDomain(target);
	await validatePathname(target.pathname);
};

export const create = async (
	platform: App.Platform,
	params: { source?: string; target?: string }
): Promise<Webmention> => {
	let source: URL;
	try {
		source = validateURL(params.source);
	} catch (e) {
		throw new ValidationError(`invalid source: ${e.message}`);
	}
	let target: URL;
	try {
		target = validateURL(params.target);
	} catch (e) {
		throw new ValidationError(`invalid target: ${e.message}`);
	}

	if (target.href === source.href) throw new ValidationError('source and target must be different');

	await validateTarget(target);

	const webmention = {
		id: nanoid(),
		source,
		target,
		status: Status.Created,
		timestamp: new Date()
	};

	const db = provideDB(platform);
	await db.put(webmention);

	return webmention;
};

export const getById = async (platform: App.Platform, id: string): Promise<Webmention> =>
	provideDB(platform).get(id);

export const repliesTo = (to: URL) =>
	accepted.flatMap(replies).filter((reply) => reply.target.href === to.href);

export const mentionsOf = (of: URL) =>
	accepted.flatMap(mentions).filter((mention) => mention.target.href === of.href);

export const likesOf = (of: URL) =>
	accepted.flatMap(likes).filter((like) => like.target.href === of.href);

export const repostsOf = (of: URL) =>
	accepted.flatMap(reposts).filter((repost) => repost.target.href === of.href);
