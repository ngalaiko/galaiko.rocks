<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';

	export const load: Load = async ({ fetch }) => {
		const res = await fetch('/posts.json');
		if (!res.ok) return { status: 500, error: 'Something went wrong' };
		const posts = (await res.json()) as Post[];
		return { status: 200, props: { posts } };
	};
</script>

<script lang="ts">
	import { Posts } from '$lib/components';

	export let posts: Post[];
</script>

<svelte:head>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
</svelte:head>

<Posts {posts} />
