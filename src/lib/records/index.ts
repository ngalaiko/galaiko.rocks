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

export const list = async () =>
  records.map((record) => ({
    ...record,
    image: images[`./covers/${record.info.coverImage.split('/').pop()}`]
  }));
