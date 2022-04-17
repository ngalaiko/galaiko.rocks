export enum Status {
	Created = 'created',
	Accepted = 'accepted',
	Rejected = 'rejected'
}

export type Webmention = {
	id: string;
	source: string;
	target: string;
	status: Status;
	message?: string;
	timestamp: Date;
};
