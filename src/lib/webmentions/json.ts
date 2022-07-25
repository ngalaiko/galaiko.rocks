import parsed from './webmentions.json';
import parsedDev from './webmentions.dev.json';
import type { Webmention } from './types';

const acceptedList: Webmention[] = parsed.concat(import.meta.env.DEV ? parsedDev : []);

export const accepted: Webmention[] = acceptedList.filter(
	({ status }) => status === 'accepted'
);
