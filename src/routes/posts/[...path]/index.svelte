<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';
	const trimRight = (str: string, char: string) => str.replace(new RegExp(`${char}+$`), '');
	// this page is either an alias to a post - then we redirect
	// or a 404 - then we show the 404 page
	// or an error - then we error
	export const load: Load = async ({ url, fetch }) => {
		const res = await fetch(trimRight(url.pathname, '/') + '.json');
		if (!res.ok)
			return res.status === 404
				? { status: 404, error: 'Page not found' }
				: { status: 500, error: 'Something went wrong' };

		const post = (await res.json()) as Post;
		if (post.path !== url.pathname) return { status: 301, redirect: post.path };

		const replies = await fetch(url.pathname + 'replies.json').then((res) => res.json());
		const likes = await fetch(url.pathname + 'likes.json').then((res) => res.json());
		const reposts = await fetch(url.pathname + 'reposts.json').then((res) => res.json());
		return {
			status: 200,
			props: { post, replies, likes, reposts }
		};
	};
</script>

<script lang="ts">
	import { Post as SinglePost } from '$lib/components';
	import type { Reply, Like, Repost } from '$lib/webmentions';

	export let likes: Like[] = [];
	export let replies: Reply[] = [];
	export let reposts: Repost[] = [];

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

<SinglePost {post} {replies} {likes} {reposts}>
	{@html post.html}
</SinglePost>
