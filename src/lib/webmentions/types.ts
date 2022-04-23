export enum Status {
	Created = 'created',
	Accepted = 'accepted',
	Rejected = 'rejected'
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

export type Reply = {
	author: Author;
	url: URL;
	content: string;
	published?: Date;
	updated?: Date;
};
