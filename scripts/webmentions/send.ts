import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { createReadStream } from 'fs';
import type { Webmention } from '../../src/lib/webmentions/types.js';
import { nanoid } from 'nanoid';
import { all } from '../../src/lib/webmentions/microformats.js';
import { readFile } from 'fs/promises';
import path from 'path';
import readdirp from 'readdirp';
import fetch from 'node-fetch';
import { type Node, type Element, parse } from 'parse5';
import { writeJSON } from '../utils.js';

const svelteKitPrerenderedPrefix = '.svelte-kit/output/prerendered/pages';

const argv = yargs(process.argv.slice(2))
	.option('baseUrl', {
		alias: 'b',
		default: 'https://galaiko.rocks',
		describe: 'Base URL for the site',
		type: 'string'
	})
	.option('input', {
		alias: 'i',
		type: 'string',
		describe: 'Path to the directory containing html files to parse',
		default: path.resolve(`${path.dirname(process.argv[1])}../../../${svelteKitPrerenderedPrefix}`),
		demandOption: true
	})
	.option('file', {
		alias: 'f',
		type: 'string',
		description: 'The file with webmentions data',
		demandOption: true
	})
	.option('dry-run', {
		alias: 'd',
		type: 'boolean',
		description: 'Do not actually send anything',
		default: false
	}).argv;

const readExistingWebmentions = async (path: string): Promise<Webmention[]> => {
	console.log('reading', path);
	const stream = createReadStream(path).pipe(JSONStream.parse('*'));
	return new Promise((resolve, reject) => {
		const webmentions: Webmention[] = [];
		stream.on('data', (webmention: any) => {
			webmentions.push(webmention);
		});
		stream.on('end', () => {
			resolve(webmentions);
		});
		stream.on('error', (error: any) => {
			reject(error);
		});
	});
};

const loadPages = async (pathname: string) =>
	await readdirp
		.promise(pathname, {
			fileFilter: '*.html',
			type: 'files'
		})
		.then((files) => files.map((file) => file.fullPath))
		.then((files) => Promise.all(files.map(async (file) => [file, await readFile(file, 'utf8')])))
		.then((files) =>
			files
				.map(([filepath, html]) => [
					filepath.split(svelteKitPrerenderedPrefix).slice(-1)[0].replace('/index.html', '/'),
					html
				])
				.map(([path, html]) => [`${argv.baseUrl}${path}`, html])
		);

const parseMentions = (files: [string, string][]) =>
	files.flatMap(([path, html]) => all(path, html));

const parseWebmentions = (pathname: string) => loadPages(pathname).then(parseMentions);

const existingMentions = await readExistingWebmentions(argv.file);

const parseFromHeaders = (headers: Headers): string | null => {
	const link = headers.get('Link');
	if (!link) return null;
	const found = link
		.split(',')
		.map((link) => ({
			rel: link.split(';')[1].split('=')[1].replace(/['"]/g, '').trim(),
			url: link.split(';')[0].replace(/[<>]/g, '').trim()
		}))
		.filter(({ rel }) => rel === 'webmention')
		.map(({ url }) => url);
	return found ? found[0] : null;
};

const discoverFromHEAD = async (target: string): Promise<string | null> =>
	fetch(target, {
		method: 'HEAD',
		headers: { 'User-Agent': 'Webmention-Discovery/galaiko.rocks' }
	})
		.then((response) => response.headers)
		.then(parseFromHeaders);

const findFirst =
	(rel: string, ...tags: string[]) =>
	(node: Node): string | null => {
		const attrs = (node as Element).attrs || [];
		const foundRel = attrs.find(({ name }) => name === 'rel');
		const foundHref = attrs.find(({ name }) => name === 'href');
		if (tags.includes(node.nodeName) && foundRel?.value === rel) return foundHref?.value;
		const childNodes = (node as Element).childNodes ?? [];
		if (childNodes.length === 0) return null;
		for (const childNode of childNodes) {
			const result = findFirst(rel, ...tags)(childNode);
			if (result) return result;
		}
		return null;
	};

const discoverFromGET = async (target: string): Promise<string | null> =>
	fetch(target, {
		method: 'GET',
		headers: { 'User-Agent': 'Webmention-Discovery/galaiko.rocks' },
		redirect: 'follow'
	})
		.then((response) => response.text())
		.then((html) => {
			const doc = parse(html);
			return findFirst('webmention', 'link', 'a')(doc);
		});

const discoverEndpoint = async (target: string): Promise<string | null> => {
	const endpoint = (await discoverFromHEAD(target)) ?? (await discoverFromGET(target));
	if (!endpoint) return null;
	try {
		// don't change absolute links
		const parsed = new URL(endpoint);
		return parsed.href;
	} catch {
		// handle relative links
		return new URL(endpoint, new URL(target)).href;
	}
};

const send = async (toSend: { target: string; source: string }[]): Promise<Webmention[]> =>
	Promise.all(
		toSend.map(async ({ target, source }) => {
			const webmention = {
				id: nanoid(),
				sourceUrl: source,
				targetUrl: target,
				timestamp: new Date().getTime()
			};
			const endpoint = await discoverEndpoint(target);
			if (!endpoint) return { ...webmention, status: 'rejected', message: 'No endpoint found' };
			console.log(`Sending webmention to ${endpoint}`);
			if (argv.dryRun) return { ...webmention, status: 'accepted' };
			const resp = await fetch(endpoint, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/x-www-form-urlencoded',
					'User-Agent': 'Webmention/galaiko.rocks'
				},
				body: `source=${source}&target=${target}`
			});
			if (!resp.ok)
				return {
					...webmention,
					status: 'rejected',
					message: `POST ${endpoint}: ${resp.status} ${resp.statusText}`
				};
			const location = resp.headers.get('Location');
			const message = location
				? `POST ${endpoint}: ${resp.status}, location: ${location}`
				: `POST ${endpoint}: ${resp.status}`;
			const status = resp.status === 201 ? 'created' : 'accepted';
			return { ...webmention, status, message };
		})
	);

const filterOutSent = (all: Webmention[]) => (parsed: { source: string; target: string }[]) =>
	parsed.filter(
		({ source, target }) =>
			!all.some(({ sourceUrl, targetUrl }) => sourceUrl === source && targetUrl === target)
	);

const storeSent = (all: Webmention[]) => async (sent: Webmention[]) => {
	if (argv.dryRun) return;
	await writeJSON(argv.file)([...all, ...sent]);
};

await parseWebmentions(argv.input)
	.then(filterOutSent(existingMentions))
	.then(send)
	.then(storeSent(existingMentions))
	.then(() => console.log('all done!'));
