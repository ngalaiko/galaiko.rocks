import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { createReadStream } from 'fs';
import { mf2 } from 'microformats-parser';
import { type Webmention } from '../../src/lib/webmentions/types.js';
import { readFile } from 'fs/promises';
import path from 'path';
import readdirp from 'readdirp';
import type { ParsedDocument, RelUrls } from 'microformats-parser/dist/types';

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
		default: path.resolve(
			`${path.dirname(process.argv[1])}../../../.svelte-kit/output/prerendered`
		),
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

const loadHtmlFiles = async (pathname: string) =>
	await readdirp
		.promise(pathname, {
			fileFilter: '*.html',
			type: 'files'
		})
		.then((files) => files.map((file) => file.fullPath))
		.then((files) => Promise.all(files.map(async (file) => [file, await readFile(file, 'utf8')])));

const extractRelUrls = (files: [string, string][]) =>
	files
		.map(([file, html]) => [file, mf2(html, { baseUrl: argv.baseUrl })])
		.map(([file, parsed]) => [file, parsed['rel-urls']])
		.map(([file, relUrls]) => [file, Object.keys(relUrls)] as [string, string[]])
		.map(([file, relUrls]) => [file, relUrls.map((relUrl) => new URL(relUrl))]);

const filterExternalUrls = (origin: string) => (files: [string, URL[]][]) =>
	files.map(([file, urls]) => [file, urls.filter((url) => url.origin !== origin)]);

// TODO:
//  * better detection of outgoing webmentions based on microformats
//  * webmention endpoint discovery
//  * store outgoing webmentions in a file
const parseWebmentions = (pathname: string) =>
	loadHtmlFiles(pathname).then(extractRelUrls).then(filterExternalUrls(argv.baseUrl));

parseWebmentions(argv.input).then((s) => console.log(s));

// readExistingWebmentions(argv.file)
// 	.then(extractWebmentions)
// 	.then((mm) => console.log(mm));
