export enum Status {
	Created = 'created',
	Accepted = 'accepted',
	Rejected = 'rejected',
	Removed = 'removed'
}

export type Parsed = {
	body: string;
	contentType: string;
};

export type Webmention = {
	id: string;
	sourceUrl: string;
	parsedSource?: Parsed;
	targetUrl: string;
	parsedTarget?: Parsed;
	status: Status;
	message?: string;
	timestamp: number;
};

export type Author = {
	picture?: string;
	name?: string;
	url: string;
};

export type Repost = {
	source: string;
	target: string;
	author: Author;
	timestamp: number;
};

export type Mention = {
	source: string;
	target: string;
	author: Author;
	timestamp: number;
};

export type Like = {
	source: string;
	target: string;
	author: Author;
	timestamp: number;
};

export type Reply = {
	author: Author;
	source: string;
	target: string;
	content: string;
	published: number;
	updated?: number;
};
