import { nanoid } from 'nanoid';
import { Status, type Webmention } from './types';

const validDomains = {
	'galaiko.rocks': true
};

const validateDomain = (url: URL) => {
	if (!validDomains[url.hostname]) {
		throw new Error(`unsupported domain: "${url.hostname}"`);
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

	validateDomain(target);

	// todo: check if target.path

	const webmention = {
		id: nanoid(),
		source,
		target,
		status: Status.Created,
		timestamp: new Date()
	};

	await platform.env.WEB_MENTIONS.put(webmention.id, JSON.stringify(webmention));

	return webmention;
};

export class NotFoundError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'NotFoundError';
	}
}

export const get = async (platform: App.Platform, id: string): Promise<Webmention> => {
	if (id.length === 0) throw new NotFoundError('not found');
	const webmention = await platform.env.WEB_MENTIONS.get<Webmention>(id, 'json');
	if (!webmention) throw new NotFoundError(`webmention ${id} not found`);
	return webmention;
};

export * from './types';
