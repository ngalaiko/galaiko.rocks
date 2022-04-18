import type { Webmention } from '../types';

export class NotFoundError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'NotFoundError';
	}
}

export interface IDatabase {
	get(key: string): Promise<Webmention>;
	put(value: Webmention): Promise<void>;
}

export const provideDB = (platform: App.Platform): IDatabase => {
	return import.meta.env.DEV ? new FileKV() : new CloudflareKV(platform.env.WEB_MENTIONS);
};

class FileKV implements IDatabase {
	private readonly filename: string = '../../data/webmentions.dev.json';

	async #readFileContent() {
		const { readFileSync } = await import('fs');
		const { dirname, join } = await import('path');
		const dir = dirname(import.meta.url);
		const filename = join(dir, this.filename).replace('file:', '');

		const data = readFileSync(filename).toString();
		if (data === '') {
			return [];
		}
		return JSON.parse(data);
	}

	async #writeFileContent(content: any) {
		const { writeFileSync } = await import('fs');
		const { dirname, join } = await import('path');
		const dir = dirname(import.meta.url);
		const filename = join(dir, this.filename).replace('file:', '');

		writeFileSync(filename, JSON.stringify(content));
	}

	async get(key: string): Promise<Webmention> {
		const db = await this.#readFileContent();
		const found = db.find(({ id }) => id === key);
		if (!found) {
			throw new NotFoundError(`Could not find ${key}`);
		}
		return found;
	}

	async put(value: Webmention): Promise<void> {
		const db = await this.#readFileContent();
		db.push(value);
		this.#writeFileContent(db);
	}
}

class CloudflareKV implements IDatabase {
	private readonly ns: KVNamespace;

	constructor(ns: KVNamespace) {
		this.ns = ns;
	}

	async get(key: string): Promise<Webmention> {
		const webmention = await this.ns.get(key);
		if (!webmention) throw new NotFoundError(`webmention ${key} not found`);
		return JSON.parse(webmention);
	}

	async put(value: Webmention): Promise<void> {
		return this.ns.put(value.id, JSON.stringify(value));
	}
}
