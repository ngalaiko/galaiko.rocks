import { spawn } from 'child_process';
import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { writeFile, createReadStream } from 'fs';
import type { Webmention } from '../../src/lib/webmentions';

const argv = yargs(process.argv.slice(2))
	.usage('Usage: $0 <command> [options]')
	.description('Download webmentions from cloudflare to a local file')
	.option('namespace-id', {
		alias: 'n',
		describe: 'namespace id',
		type: 'string',
		demandOption: true,
		default: '04717db6466d4700b257589fec573c01'
	})
	.option('file', {
		alias: 'f',
		type: 'string',
		description: 'The file with webmentions data',
		demandOption: true
	}).argv;

const wrangler = (...args: string[]): Promise<string> =>
	new Promise((resolve, reject) => {
		const child = spawn('wrangler', [...args]);
		let error = '';
		let output = '';
		child.stderr.on('data', (data) => (error += data));
		child.stderr.on('end', () => {
			if (error) {
				reject(error);
			}
		});
		child.stdout.on('data', (data) => (output += data));
		child.stdout.on('end', () => {
			resolve(output);
		});
	});

const listKeys = async (): Promise<string[]> => {
	console.log('listing keys');
	return wrangler('kv:key', 'list', `--namespace-id=${argv.namespaceId}`)
		.then(JSON.parse)
		.then((keys) => keys.map(({ name }) => name));
};

const downloadKey = async (key: string): Promise<any> => {
	console.log('downloading', key);
	return wrangler('kv:key', 'get', `--namespace-id=${argv.namespaceId}`, key);
};

const writeJSON =
	(path: string) =>
	(data: any): Promise<void> =>
		new Promise((resolve, reject) => {
			console.log('writing to', path);
			writeFile(path, JSON.stringify(data), (error) => {
				if (error) {
					reject(error);
				} else {
					resolve();
				}
			});
		});

const readExistingWebmentions = async (path: string): Promise<Webmention[]> => {
	console.log('reading', path);
	return new Promise((resolve, reject) => {
		const stream = createReadStream(path).pipe(JSONStream.parse('*'));
		const webmentions: Webmention[] = [];
		stream.on('data', (webmention: Webmention) => {
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

const downloadKeys = async (keys: string[]): Promise<Webmention[]> =>
	Promise.all(keys.map(downloadKey)).then((values) => values.map((value) => JSON.parse(value)));

const filterDownloadedKeys =
	(existingWebmentions: Webmention[]) =>
	async (keys: string[]): Promise<string[]> => {
		const toDownload = keys.filter(
			(key) => !existingWebmentions.find((webmention) => webmention.id === key)
		);
		console.log(toDownload.length, 'keys to download');
		return toDownload;
	};

const existingKeys = await readExistingWebmentions(argv.file);

await listKeys()
	.then(filterDownloadedKeys(existingKeys))
	.then((toDownload) => {
		if (toDownload.length === 0) {
			return;
		}
		return downloadKeys(toDownload)
			.then((newKeys) => existingKeys.concat(newKeys))
			.then(writeJSON(argv.file));
	})
	.then(() => console.log('done!'));
