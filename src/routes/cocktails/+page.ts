import type { PageLoad } from './$types';
import { list } from '$lib/cocktails';

export const load: PageLoad = async () => list().then((cocktails) => ({ cocktails }));
