import { nanoid } from 'nanoid';
import { provideDB } from './database';
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

	// todo: check if target.path is not 404

	const webmention = {
		id: nanoid(),
		source,
		target,
		status: Status.Created,
		timestamp: new Date()
	};

	const db = provideDB(platform);
	db.put(webmention);

	return webmention;
};

export const get = async (platform: App.Platform, id: string): Promise<Webmention> =>
	provideDB(platform).get(id);

export * from './types';
