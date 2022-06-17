import yargs from 'yargs';
import fetch from 'node-fetch';
import { addSeconds, differenceInSeconds } from 'date-fns';
import { writeJSON } from '../../utils.js';

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
		type: 'string'
		// demandOption: true
	});

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
			authorization: `bearer ${argv.argv.apiToken}`,
			'x-auth-email': argv.argv.email,
			'content-Type': 'application/json'
		},
		body: JSON.stringify({
			query,
			variables: variables({ from, to }),
			operationName
		})
	}).then(async (resp) => (await resp.json()) as Response);

const generatePeriods = ({ from, to, interval }: { from: Date; to: Date; interval: number }) => {
	const periods = [];
	const now = new Date();
	for (let i = 0; i < Math.ceil(differenceInSeconds(to, from) / interval); i++) {
		const from = addSeconds(now, -interval * (i + 1));
		const to = addSeconds(now, -interval * i);
		periods.push({ from, to });
	}
	return periods;
};

Promise.all(
	generatePeriods({
		from: new Date('2022-04-03T15:47:36.061747Z'),
		to: new Date(),
		interval: 3600 // 1 hour
	}).map(async ({ from, to }) => {
		const { data, errors } = await list({ from, to }).catch((err) => ({
			errors: [err],
			data: null
		}));
		if (!data) return { from, to, metrics: {} };
		if (errors) throw new Error(errors.map((e) => e.message).join('\n'));
		const metrics = data.viewer.accounts[0].topPaths.reduce(
			(acc, { metric: { requestPath }, count }) => ({ ...acc, [requestPath]: count }),
			{}
		);
		return { from, to, metrics };
	})
)
	.then((periods) =>
		periods.reduce((acc, { metrics }) => {
			Object.entries(metrics).forEach(([key, value]) => {
				if (acc[key]) {
					acc[key] += value;
				} else {
					acc[key] = value;
				}
			});
			return acc;
		}, {})
	)
	.then((metrics) =>
		Object.entries(metrics).sort((a: [string, number], b: [string, number]) => b[1] - a[1])
	)
	.then(console.log);
