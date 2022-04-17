import { spawn } from 'child_process';
import JSONStream from 'JSONStream';
import yargs from 'yargs';
import { writeFile, createReadStream } from 'fs';
import { Status, type Webmention } from '../../src/lib/webmentions/types.js';
import { compareAsc } from 'date-fns';

const argv = yargs(process.argv.slice(2)).usage('Usage: $0 <command> [options]').option('file', {
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

const namespaceId = '04717db6466d4700b257589fec573c01';

const update = async (webmention: Webmention): Promise<Webmention> => {
	console.log('uploading', webmention.id);
	await wrangler(
		'kv:key',
		'put',
		`--namespace-id=${namespaceId}`,
		webmention.id,
		JSON.stringify(webmention)
	);
	return webmention;
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
	const stream = createReadStream(path).pipe(JSONStream.parse('*'));
	return new Promise((resolve, reject) => {
		const webmentions: Webmention[] = [];
		stream.on('data', (webmention: Webmention) => {
			webmentions.push({ ...webmention, timestamp: new Date(webmention.timestamp) });
		});
		stream.on('end', () => {
			resolve(webmentions);
		});
		stream.on('error', (error: any) => {
			reject(error);
		});
	});
};

// group webmentions by combination of target and source
// members of a group are sorted by date where the earliest is first
const groupByTargetSource = (webmentions: Webmention[]): Webmention[][] => {
	const groups: Webmention[][] = [];
	webmentions.forEach((webmention) => {
		const group = groups.find(
			([first]) => first?.target === webmention.target && first?.source === webmention.source
		);
		if (group) {
			group.push(webmention);
			group.sort((a, b) => compareAsc(a.timestamp, b.timestamp));
		} else {
			groups.push([webmention]);
		}
	});
	return groups;
};

// mark duplicates by combination of target and source
// if duplicate is found, webmention with earliest date is considered the original,
// the rest are rejected with the message 'duplicate'
const markDuplicates = async (webmentions: Webmention[]): Promise<Webmention[]> =>
	Promise.all(
		groupByTargetSource(webmentions).flatMap((group) =>
			group.map(async (webmention, index) => {
				return index === 0
					? webmention
					: await update({
							...webmention,
							status: Status.Rejected,
							message: `duplicate of ${group[0].id}`
					  });
			})
		)
	);

readExistingWebmentions(argv.file).then(markDuplicates).then(writeJSON(argv.file));
