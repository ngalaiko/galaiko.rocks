import { nanoid } from 'nanoid';
import { Status, type Webmention } from './types';

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

const validateURL = (url?: string) => {
	if (!url || !url.length) {
		throw new Error('must not be empty');
	}
	const parsed = parseURL(url);
	validateProtocol(parsed);
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
	try {
		validateURL(params.source);
	} catch (e) {
		throw new ValidationError(`invalid source: ${e.message}`);
	}
	try {
		validateURL(params.target);
	} catch (e) {
		throw new ValidationError(`invalid target: ${e.message}`);
	}

	const webmention = {
		id: nanoid(),
		source: params.source,
		target: params.target,
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
