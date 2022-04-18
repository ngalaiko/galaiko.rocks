import parsed from '../data/webmentions.json';
import parsedDev from '../data/webmentions.dev.json';
import { type Webmention, type Parsed, Status } from './types';

const parsedFromJSON = (i: any): Parsed | URL => {
	return i instanceof Object
		? {
				url: new URL(i.url),
				contentType: i.contentType,
				body: i.body
		  }
		: new URL(i as string);
};

const webmentionFromJSON = (mention: any): Webmention => ({
	target: parsedFromJSON(mention.target),
	source: parsedFromJSON(mention.source),
	timestamp: new Date(mention.timestamp),
	status: mention.status as Status,
	id: mention.id
});

const acceptedList: Webmention[] = parsed
	.map(webmentionFromJSON)
	.concat(import.meta.env.DEV ? parsedDev.map(webmentionFromJSON) : []);

export const accepted: Webmention[] = acceptedList.filter(
	({ status }) => status === Status.Accepted
);
