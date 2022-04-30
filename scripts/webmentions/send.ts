import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { createReadStream } from 'fs';
import type { Webmention } from '../../src/lib/webmentions/types.js';
import { all } from '../../src/lib/webmentions/microformats.js';
import { readFile } from 'fs/promises';
import path from 'path';
import readdirp from 'readdirp';

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
		.then((files) => Promise.all(files.map(async (file) => [file, await readFile(file, 'utf8')])))
		.then((files) =>
			files
				.map(([filepath, html]) => [filepath.split('/src/routes/').slice(-1)[0], html])
				.map(([path, html]) => [`${argv.baseUrl}/${path}`, html])
		);

const getMentions = (files: [string, string][]) => files.flatMap(([path, html]) => all(path, html));

// TODO:
//  * webmention endpoint discovery
//  * store outgoing webmentions in a file
const parseWebmentions = (pathname: string) => loadHtmlFiles(pathname).then(getMentions);

parseWebmentions(argv.input).then((s) => console.log(s));

// readExistingWebmentions(argv.file)
// 	.then(extractWebmentions)
// 	.then((mm) => console.log(mm));
