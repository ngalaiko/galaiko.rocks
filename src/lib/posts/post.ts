export type Post = {
	title: string;
	date: Date;
	path: string;
	html: string;
	aliases: string[];
	hidden: boolean;
};
