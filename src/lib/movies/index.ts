import movies from './data.json';

export type Movie = {
  title: string;
  watchedDate: string;
  href: string;
  rewatch: boolean;
  poster: string;
};

export const list = (): Movie[] => movies;
