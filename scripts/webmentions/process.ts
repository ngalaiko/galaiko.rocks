import { spawn } from 'child_process';
import JSONStream from 'JSONStream';
import yargs from 'yargs';
import parse5, { type Node, type Element } from 'parse5';
import { writeFile, createReadStream } from 'fs';
import { Status, type Webmention, type Parsed } from '../../src/lib/webmentions/types.js';
import { compareAsc } from 'date-fns';
import fetch, { type Response } from 'node-fetch';

const argv = yargs(process.argv.slice(2))
	.usage('Usage: $0 <command> [options]')
	.description('Process local webmentions and upload results to remove kv')
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
	})
	.option('dev', {
		alias: 'd',
		type: 'boolean',
		description: 'Do not send data to cloudflare',
		default: false
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

const update = async (webmention: Webmention): Promise<Webmention> => {
	if (argv.dev) return webmention;
	console.log('uploading', webmention.id);
	await wrangler(
		'kv:key',
		'put',
		`--namespace-id=${argv.namespace_id}`,
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
			webmentions.push({
				...webmention,
				timestamp: new Date(webmention.timestamp),
				source: webmention.source,
				target: webmention.target
			});
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

class SourceValidationError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'SourceValidationError';
	}
}

const href = (u: URL | Parsed | string): string => {
	if ((u as Parsed).url) return (u as Parsed).url.href;
	if ((u as URL).href) return (u as URL).href;
	return u as string;
};

const validatePlainTextSource = async (webmention: Webmention, text: string): Promise<string> => {
	if (text.includes(href(webmention.target))) return text;
	throw new SourceValidationError(
		`${href(webmention.source)} does not contain mention of ${href(webmention.target)}`
	);
};

const jsonContainsValue =
	(lookFor: string) =>
	(json: any): boolean => {
		if (json instanceof Array) {
			return json.some(jsonContainsValue(lookFor));
		}
		if (json instanceof Object) {
			return Object.values(json).some(jsonContainsValue(lookFor));
		}
		return json === lookFor;
	};

const validateJSONSource = async (webmention: Webmention, json: string): Promise<string> => {
	const targetUrl = href(webmention.target);
	if (jsonContainsValue(targetUrl)(JSON.parse(json))) return json;
	throw new SourceValidationError(
		`${href(webmention.source)} does not contain mention of ${href(webmention.target)}`
	);
};

const hasLink =
	(href: string) =>
	(node: Node): boolean => {
		const attributes = (node as Element).attrs;
		if (
			attributes &&
			attributes.some(({ name, value }) => ['href', 'src'].includes(name) && value === href)
		) {
			return true;
		}
		const childNodes = (node as Element).childNodes;
		return childNodes ? childNodes.some(hasLink(href)) : false;
	};

const validateHtmlSource = async (webmention: Webmention, html: string): Promise<string> => {
	const targetUrl = href(webmention.target);
	if (hasLink(targetUrl)(parse5.parse(html))) return html;
	throw new SourceValidationError(
		`${href(webmention.source)} does not contain mention of ${href(webmention.target)}`
	);
};

// valdiate webmention source and return it if valid
const validateSource = async (
	webmention: Webmention,
	sourceResponse: Response
): Promise<string> => {
	const contentType = sourceResponse.headers.get('content-type');
	if (!contentType) throw new SourceValidationError(`no content-type header found in response`);
	if (contentType.includes('text/html')) {
		return validateHtmlSource(webmention, await sourceResponse.text());
	} else if (contentType.includes('application/json')) {
		return validateJSONSource(webmention, await sourceResponse.text());
	} else if (contentType.includes('text/plain')) {
		return validatePlainTextSource(webmention, await sourceResponse.text());
	} else {
		throw new SourceValidationError(`unsupported content-type ${contentType}`);
	}
};

// downloadSource downloads the source html of the webmention and updates the status to accepted if successful
const downloadSource = async (webmention: Webmention): Promise<Webmention> => {
	// only dowload created webmentions
	if (webmention.status !== Status.Created) return webmention;
	const sourceHref = href(webmention.source);
	console.log('downloading', sourceHref);
	const response = await fetch(sourceHref, {
		headers: {
			Accept: 'text/html, application/json, text/plain',
			'User-Agent': 'Webmention/1.0 (crawler; https://galaiko.rocks)'
		},
		redirect: 'follow'
	});
	if (!response.ok) {
		console.log(`${webmention.id} rejected: ${response.status} ${response.statusText}`);
		return update({
			...webmention,
			status: Status.Rejected,
			message: `failed to GET ${webmention.source.toString()}: ${response.status} ${
				response.statusText
			}`
		});
	}

	try {
		const body = await validateSource(webmention, response);
		console.log(`${webmention.id} accepted`);
		return update({
			...webmention,
			status: Status.Accepted,
			source: {
				url: webmention.source as URL,
				contentType: response.headers.get('content-type'),
				body
			}
		});
	} catch (error) {
		if (error instanceof SourceValidationError) {
			console.log(`${webmention.id} rejected: ${error.message}`);
			return update({ ...webmention, status: Status.Rejected, message: error.message });
		}
		throw error;
	}
};

const processWebmentions = async (webmentions: Webmention[]): Promise<Webmention[]> =>
	Promise.all(webmentions.map(downloadSource));

readExistingWebmentions(argv.file).then(processWebmentions).then(writeJSON(argv.file));
