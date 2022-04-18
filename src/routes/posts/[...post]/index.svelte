<script context="module" lang="ts">
	export const load = async ({ params, fetch }) => {
		const { post } = params;
		const url = post.endsWith('/') ? `/posts/${post.slice(0, -1)}.json` : `/posts/${post}.json`;
		const res = await fetch(url);
		if (res.ok) {
			// This will only happen for posts that were not found on the real path, but
			// were found through the alias, thus redirect.
			const post = await res.json();
			return { status: 301, redirect: post.path };
		} else {
			if (res.status === 404) {
				return { status: 200 }; // render not found
			}
			return {
				status: 500,
				error: new Error(`Could not fetch ${url}`)
			};
		}
	};
</script>

<script>
	import NotFound from '$lib/components/NotFound.svelte';
</script>

<NotFound />
