import yargs from 'yargs';
import { writeJSON, readJSON } from '../utils.js';
import { parseStringPromise } from 'xml2js';
import type { Movie } from '../../src/lib/movies/index.js';

const argv = yargs(process.argv.slice(2))
  .usage('Usage: $0 <command> [options]')
  .option('username', {
    alias: 'u',
    describe: 'The username to fetch',
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

const read = async (path: string): Promise<Movie[]> => readJSON(path);

const download = async (): Promise<Movie[]> => {
  const xml = await fetch('https://letterboxd.com/ngalaiko/rss/');
  const text = await xml.text();
  const json = await parseStringPromise(text);
  return Promise.all(
    json.rss.channel[0].item.map(async (item: any) => {
      const title = item['letterboxd:filmTitle'][0];
      const watchedDate = item['letterboxd:watchedDate'][0];
      const rewatch = item['letterboxd:rewatch'][0] === 'Yes';
      const href = item.link[0];
      const description = await parseStringPromise(item.description[0]);
      const poster = description.p.img[0]['$'].src;
      return {
        title,
        watchedDate,
        rewatch,
        href,
        poster
      };
    })
  );
};

Promise.all([read(argv.output), download()])
  .then(([read, downloaded]) => {
    const toAdd = downloaded.filter(
      (downloaded) => !read.some((read) => read.href === downloaded.href)
    );
    return [...read, ...toAdd];
  })
  .then(writeJSON(argv.output));
