import type { PageLoad } from './$types';
import { list } from '$lib/restaurants-and-cafes';

export const load: PageLoad = async () => list().then((places) => ({ places }));
export const prerender = false;
