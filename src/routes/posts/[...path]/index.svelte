<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';

	const trimRight = (str: string, char: string) => str.replace(new RegExp(`${char}+$`), '');

	// this page is either an alias to a post - then we redirect
	// or an error - then we error
	export const load: Load = async ({ url, fetch }) => {
		const res = await fetch(trimRight(url.pathname, '/') + '.json');
		if (res.ok) {
			const post = (await res.json()) as Post;
			return { status: 301, redirect: post.path };
		} else {
			return res.status === 404
				? { status: 404, error: 'Page not found' }
				: { status: 500, error: 'Something went wrong' };
		}
	};
</script>
