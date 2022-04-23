import parsed from '../data/webmentions.json';
import parsedDev from '../data/webmentions.dev.json';
import { type Webmention, Status } from './types';
import { webmentionFromJSON } from './types';

const acceptedList: Webmention[] = parsed
	.map(webmentionFromJSON)
	.concat(import.meta.env.DEV ? parsedDev.map(webmentionFromJSON) : []);

export const accepted: Webmention[] = acceptedList.filter(
	({ status }) => status === Status.Accepted
);
