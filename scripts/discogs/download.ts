import yargs from 'yargs';
import { writeJSON } from '../utils.js';
import type { Record } from '../../src/lib/records/index.js';

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

download(argv.argv.apiToken)
	.then((records: any): Record[] =>
		records
			.filter((record: any) => record.basic_information.cover_image)
			.filter((record: any) => record.basic_information.thumb)
			.map((raw: any) => ({
				artist: {
					name: raw.basic_information.artists[0].name
				},
				info: {
					id: raw.basic_information.id,
					title: raw.basic_information.title,
					coverImage: raw.basic_information.cover_image
				}
			}))
	)
	.then(writeJSON(argv.argv.output));
