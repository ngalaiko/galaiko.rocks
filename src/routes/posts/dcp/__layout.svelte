<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';

	const trimRight = (str: string, char: string) => str.replace(new RegExp(`${char}+$`), '');

	export const load: Load = async ({ url, fetch }) => {
		const res = await fetch(trimRight(url.pathname, '/') + '.json');
		if (!res.ok)
			return res.status === 404
				? { status: 404, error: 'Page not found' }
				: { status: 500, error: 'Something went wrong' };

		const post = (await res.json()) as Post;
		if (post.path !== url.pathname) return { status: 301, redirect: post.path };

		return {
			status: 200,
			props: { post }
		};
	};
</script>

<script lang="ts">
	import { Post as SinglePost } from '$lib/components';

	export let post: Post;
</script>

<svelte:head>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
	<title>{post.title}</title>
	{#if post.previous}
		<link rel="prev" href={post.previous.path} />
	{/if}
	{#if post.next}
		<link rel="next" href={post.next.path} />
	{/if}
</svelte:head>

<SinglePost {post}>
	<slot />
</SinglePost>
