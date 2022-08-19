import { redirect, error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { findByPathname } from '$lib/posts';

// this page is either an alias to a post - then we redirect
// or a 404 - then we show the 404 page
// or an error - then we error
export const load: PageLoad = async ({ url }) => {
	const post = await findByPathname(url.pathname);
	if (!post) {
		throw error(404);
	} else if (post.path !== url.pathname) {
		throw redirect(301, post.path);
	} else {
		return { post };
	}
};