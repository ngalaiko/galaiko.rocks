export enum Status {
	Created = 'created',
	Accepted = 'accepted',
	Rejected = 'rejected',
	Removed = 'removed'
}

export type Parsed = {
	url: URL;
	body: string;
	contentType: string;
};

export type Webmention = {
	id: string;
	source: URL | Parsed;
	target: URL | Parsed;
	status: Status;
	message?: string;
	timestamp: Date;
};

export type Author = {
	picture?: URL;
	name?: string;
	url: string;
};

export type Like = {
	source: URL;
	target: URL;
	author: Author;
	timestamp: Date;
};

export type Reply = {
	author: Author;
	source: URL;
	target: URL;
	content: string;
	published: Date;
	updated?: Date;
};

const parsedFromJSON = (i: any): Parsed | URL =>
	i instanceof Object
		? {
				url: new URL(i.url),
				contentType: i.contentType,
				body: i.body
		  }
		: new URL(i as string);

export const webmentionFromJSON = (mention: any): Webmention => ({
	target: parsedFromJSON(mention.target),
	source: parsedFromJSON(mention.source),
	timestamp: new Date(mention.timestamp),
	status: mention.status as Status,
	id: mention.id
});
