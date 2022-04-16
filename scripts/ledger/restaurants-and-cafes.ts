import { spawn } from 'child_process';
import { writeFile } from 'fs';
import csv from '@fast-csv/parse';
import { startOfDay } from 'date-fns';
import currency from 'currency.js';
import { addYears } from 'date-fns';
import yargs from 'yargs';

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
		description: 'Number of places to export'
	})
	.option('output', {
		alias: 'o',
		type: 'string',
		description: 'Output file',
		demandOption: true
	}).argv;

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

export const groupBy = <T, K extends keyof any>(list: T[], getKey: (item: T) => K) =>
	list.reduce((previous, currentItem) => {
		const group = getKey(currentItem);
		if (!previous[group]) previous[group] = [];
		previous[group].push(currentItem);
		return previous;
	}, {} as Record<K, T[]>);

await exportTransactions(argv.file)
	.then((transactions) =>
		Object.entries(groupBy(transactions, (t: Transaction) => t.payee))
			.map(([payee, transactions]) => ({
				payee,
				count: transactions.length,
				currency: transactions[0].currency,
				amount: transactions.reduce((sum, t) => sum.add(t.amount), currency(0))
			}))
			.sort((a, b) => b.count - a.count)
			.slice(0, argv.number || 10)
	)
	.then(writeJSON(argv.output))
	.then(() => console.log('done!'));
