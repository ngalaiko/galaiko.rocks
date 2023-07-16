import { spawn } from 'child_process';
import csv from '@fast-csv/parse';
import { startOfDay } from 'date-fns';
import currency from 'currency.js';
import { addYears } from 'date-fns';
import yargs from 'yargs';
import { writeJSON } from '../utils.js';
import locations from './locations.js';

const argv = yargs(process.argv.slice(2))
	.usage('Usage: $0 <command> [options]')
	.option('file', {
		alias: 'f',
		type: 'string',
		description: 'The ledger file to use'
	})
	.option('number', {
		alias: 'n',
		type: 'number',
		description: 'Minimum number of visits'
	})
	.option('output', {
		alias: 'o',
		type: 'string',
		description: 'Output file',
		demandOption: true
	})
	.parseSync();

type Transaction = { date: Date; amount: currency; payee: string; currency: string };

const log = (...args: any[]) => {
	console.log(...args);
	return args;
};

const hledger = (args: string[]): Promise<any[]> =>
	new Promise((resolve, reject) => {
		const rows = [];
		const parseCSV = csv
			.parse({ headers: true })
			.on('error', reject)
			.on('data', (row) => rows.push(row))
			.on('end', () => resolve(rows));
		const child = spawn('hledger', [...args]);
		let error = '';
		child.stderr.on('data', (data) => (error += data));
		child.stderr.on('end', () => {
			if (error) {
				reject(error);
			}
		});
		child.stdout.pipe(parseCSV);
	});

const exportTransactions = async (file?: string) => {
	log('exporting transactions...');
	const withFile = file ? ['--file', file] : [];
	const params = [
		'register',
		'--value=then,SEK',
		'--output-format=csv',
		'expenses:Food:Restaurants & Cafes$',
		'--begin',
		addYears(new Date(), -1).toISOString().slice(0, 10)
	];
	return await hledger(params.concat(withFile)).then((rows) =>
		rows.map((row: any): Transaction => {
			const { date, description, amount } = row;
			const amountParts = amount.split(' ');
			return {
				date: startOfDay(new Date(date)),
				payee: description.split('|')[0],
				amount: currency(amountParts[0]),
				currency: amountParts[1]
			};
		})
	);
};

export const groupBy = <T, K extends keyof any>(list: T[], getKey: (item: T) => K) =>
	list.reduce((previous, currentItem) => {
		const group = getKey(currentItem);
		if (!previous[group]) previous[group] = [];
		previous[group].push(currentItem);
		return previous;
	}, {} as Record<K, T[]>);

exportTransactions(argv.file)
	.then((transactions) =>
		Object.entries(groupBy(transactions, (t: Transaction) => t.payee))
			.map(([payee, transactions]) => ({
				payee,
				// filter out negative transactions as they are refunds
				count: transactions.filter((t) => t.amount.intValue > 0).length,
				currency: transactions[0].currency,
				amount: transactions.reduce((sum, t) => sum.add(t.amount), currency(0))
			}))
			.filter((e) => e.count >= argv.number)
			.sort((a, b) => b.count - a.count)
			.map((entry) => {
				const payee = entry.payee.trim();
				const location = locations[payee];
				if (!location) throw new Error(`${entry.payee}: location missing`);
				return { ...entry, location };
			})
	)
	.then(writeJSON(argv.output))
	.then(() => console.log('done!'));
