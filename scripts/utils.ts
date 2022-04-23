import { writeFile } from 'fs';

export const writeJSON =
	(path: string) =>
	(data: any): Promise<void> =>
		new Promise((resolve, reject) => {
			console.log('writing to', path);
			writeFile(path, JSON.stringify(data, null, '  '), (error) => {
				if (error) {
					reject(error);
				} else {
					resolve();
				}
			});
		});
