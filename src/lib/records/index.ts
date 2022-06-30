import records from './data.json';

export type Record = {
	artist: {
		name: string;
	};
	info: {
		id: number;
		title: string;
		coverImage?: string;
	};
};

export const list = () => records as Record[];
