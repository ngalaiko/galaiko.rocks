import yargs from 'yargs';
import fetch from 'node-fetch';
import { writeFile } from 'fs';

const argv = yargs(process.argv.slice(2))
	.usage('Usage: $0 <command> [options]')
	.option('api-token', {
		alias: 't',
		describe: 'The token to use for authentication',
		type: 'string',
		demandOption: true
	})
	.option('output', {
		alias: 'o',
		describe: 'The output file',
		type: 'string',
		demandOption: true
	});

const download = async (token: string, page = 1) =>
	fetch(
		`https://api.discogs.com/users/ngalaiko/collection/folders/0/releases?sort=artist&page=${page}`,
		{
			headers: {
				Authorization: `Discogs token=${token}`,
				Accept: 'application/json'
			}
		}
	)
		.then((res) => res.json())
		.then(async (json: any) => {
			const releases = json.releases;
			const { next } = json.pagination.urls;
			return next ? [...releases, ...(await download(token, page + 1))] : releases;
		});

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

await download(argv.argv.apiToken).then(writeJSON(argv.argv.output));
