import parsed from '../data/webmentions.json';
import parsedDev from '../data/webmentions.dev.json';
import { type Webmention, Status } from './types';

const acceptedList: Webmention[] = parsed.concat(import.meta.env.DEV ? parsedDev : []);

export const accepted: Webmention[] = acceptedList.filter(
	({ status }) => status === Status.Accepted
);
