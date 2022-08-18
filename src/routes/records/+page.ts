import { list } from '$lib/records';

import type { PageLoad } from './$types';

export const load: PageLoad = async () => list().then((records) => ({ records }));
