import type { PageLoad } from './$types';
import { list } from '$lib/posts';

export const load: PageLoad = async () => ({ posts: await list() });
