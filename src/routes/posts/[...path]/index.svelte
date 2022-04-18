<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';
	export const load: Load = async ({ params, fetch }) => {
		const { path } = params;
		const url = path.endsWith('/') ? `/posts/${path.slice(0, -1)}.json` : `/posts/${path}.json`;
		const res = await fetch(url);
		if (res.ok) {
			// This route only mathes for posts that were not found on the real path, but
			// got here through ...post catchall match, thus - redirect them
			const post = (await res.json()) as Post;
			return { status: 301, redirect: post.path };
		} else {
			if (res.status === 404) {
				return { status: 200 }; // render not found
			}
			return {
				status: 500,
				body: 'Something went wrong'
			};
		}
	};
</script>

<script>
	import NotFound from '$lib/components/NotFound.svelte';
</script>

<NotFound />
