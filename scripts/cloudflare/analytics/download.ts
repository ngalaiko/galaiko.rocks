import yargs from 'yargs';
import { startOfDay, addDays } from 'date-fns';
import { writeJSON, readJSON } from '../../utils.js';

const argv = yargs(process.argv.slice(2))
	.usage('Usage: $0 <command> [options]')
	.option('api-token', {
		alias: 't',
		describe: 'The token to use for authentication',
		type: 'string',
		demandOption: true
	})
	.option('email', {
		alias: 'e',
		describe: 'The email to use for authentication',
		type: 'string',
		demandOption: true
	})
	.option('output', {
		alias: 'o',
		describe: 'The output file',
		type: 'string',
		demandOption: true
	})
	.parseSync();

const variables = ({ from, to }: { from: Date; to: Date }) => ({
	accountTag: 'd9eda3671dd279e79c22103ce1f879dd',
	filter: {
		AND: [
			{
				datetime_geq: from.toISOString(),
				datetime_leq: to.toISOString()
			},
			{
				siteTag_in: ['99fffbfd97b74b1880c23b0617f24aac']
			}
		]
	},
	order: 'sum_visits_DESC',
	limit: 1000
});

const operationName = 'GetRumAnalyticsTopPaths';

const query = `query GetRumAnalyticsTopPaths {
  viewer {
    accounts(filter: {accountTag: $accountTag}) {
      topPaths: rumPageloadEventsAdaptiveGroups(
        filter: $filter, 
        limit: $limit, 
        orderBy: [$order],
	  ) {
        count
        metric: dimensions {
          requestPath
        }
      }
    }
  }
}`;

type Response = {
	errors?: { code: number; message: string }[];
	data?: {
		viewer: {
			accounts: {
				topPaths: {
					count: number;
					metric: {
						requestPath: string;
					};
				}[];
			}[];
		};
	};
};

const list = async ({ from, to }: { from: Date; to: Date }): Promise<Response> =>
	fetch('https://api.cloudflare.com/client/v4/graphql', {
		method: 'POST',
		headers: {
			authorization: `bearer ${argv.apiToken}`,
			'x-auth-email': argv.email,
			'content-Type': 'application/json'
		},
		body: JSON.stringify({
			query,
			variables: variables({ from, to }),
			operationName
		})
	}).then(async (resp) => (await resp.json()) as Response);

const download = async () => {
	const to = startOfDay(new Date());
	const from = addDays(to, -1);
	const { data, errors } = await list({ from, to }).catch((err) => ({
		errors: [err],
		data: null
	}));
	if (!data) return { from, to, metrics: {} };
	if (errors) throw new Error(errors.map((e: any) => e.message).join('\n'));
	const metrics = data.viewer.accounts[0].topPaths.reduce(
		(acc, { metric: { requestPath }, count }) => ({ ...acc, [requestPath]: count }),
		{}
	);
	return { from, to, metrics };
};

Promise.all([download(), readJSON(argv.output)])
	.then(([data, existing]) => [...existing, data])
	.then(writeJSON(argv.output));
