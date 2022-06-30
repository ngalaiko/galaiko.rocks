import records from './data.json';

export type Record = {
	artist: {
		name: string;
	};
	info: {
		id: string;
		title: string;
		coverImage?: string;
	};
};

const all = records.map((raw: any) => ({
	artist: {
		name: raw.basic_information.artists[0].name
	},
	info: {
		id: raw.basic_information.id,
		title: raw.basic_information.title,
		coverImage: raw.basic_information.cover_image
	}
}));

export const list = () => all;
