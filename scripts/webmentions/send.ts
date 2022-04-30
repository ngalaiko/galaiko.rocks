import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { createReadStream } from 'fs';
import type { Webmention } from '../../src/lib/webmentions/types.js';
import { all } from '../../src/lib/webmentions/microformats.js';
import { readFile } from 'fs/promises';
import path from 'path';
import readdirp from 'readdirp';

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

// TODO:
//  * webmention endpoint discovery
//  * store outgoing webmentions in a file
const parseWebmentions = (pathname: string) => loadPages(pathname).then(parseMentions);

const existingMentions = readExistingWebmentions(argv.file);

const filterOutSent = (urls: string[]) => (all: { source: string }[]) =>
	all.filter(({ source }) => !urls.includes(source));

const existingUrls = await existingMentions.then((webmentions) =>
	webmentions.map(({ sourceUrl }) => sourceUrl)
);

parseWebmentions(argv.input).then(filterOutSent(existingUrls)).then(console.log);
