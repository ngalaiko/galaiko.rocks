import records from './data.json';

export type Record = {
	artist: {
		name: string;
	};
	info: {
		id: number;
		title: string;
		coverImage: string;
	};
};

// covers are populated with vite-plugin-remote-assets
const images = import.meta.importGlob('./covers/*.jpeg', {
	import: 'default',
	eager: true,
	query: {
		preset: 'hd'
	}
});

export const list = () =>
	records
		.map((record) => [record, images[`./covers/${record.info.coverImage.split('/').pop()}`]]);
