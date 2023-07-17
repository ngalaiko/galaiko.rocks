import { list } from '$lib/movies';
import { compareDesc, parseISO } from 'date-fns';

import type { PageLoad } from './$types';

export const load: PageLoad = async () => ({
  movies: list()
    .map((movie) => ({
      ...movie,
      watchedDate: parseISO(movie.watchedDate)
    }))
    .sort((a, b) => compareDesc(a.watchedDate, b.watchedDate))
});
