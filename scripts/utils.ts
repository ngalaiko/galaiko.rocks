import { writeFile, readFile } from 'fs';

export const readJSON = (path: string): any =>
  new Promise((resolve, reject) => {
    readFile(path, 'utf8', (err, data) => {
      if (err) {
        reject(err);
      }
      resolve(JSON.parse(data));
    });
  });

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
