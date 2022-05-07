<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Post } from '$lib/posts';

	const trimRight = (str: string, char: string) => str.replace(new RegExp(`${char}+$`), '');

	// this is a cathall for any /post/... page, ther are a few options:
	// 1. this is post => just render it
	// 2. this is an alias for a post => redirect to the correct one
	// 3. this is a list page => render the list
	// 4. nothing => throw 404
	export const load: Load = async ({ url, fetch }) => {
		const res = await fetch(trimRight(url.pathname, '/') + '.json');
		if (!res.ok)
			return res.status === 404
				? { status: 404, error: 'Page not found' }
				: { status: 500, error: 'Something went wrong' };

		const body = (await res.json()) as Post | Post[];
		if (body instanceof Array) return { status: 200, props: { posts: body } };
		if (body.path !== url.pathname) return { status: 301, redirect: body.path };

		return {
			status: 200,
			props: {
				post: body,
				replies: await fetch(url.pathname + 'replies.json').then((res) => res.json()),
				likes: await fetch(url.pathname + 'likes.json').then((res) => res.json()),
				reposts: await fetch(url.pathname + 'reposts.json').then((res) => res.json())
			}
		};
	};
</script>

<script lang="ts">
	import { WithBorder, Post as SinglePost, Posts } from '$lib/components';
	import { Footer } from '$lib/components/molecules';
	import type { Reply, Like, Repost } from '$lib/webmentions';

	export let likes: Like[] = [];
	export let replies: Reply[] = [];
	export let reposts: Repost[] = [];

	export let post: Post = null;
	export let posts: Post[] = [];
</script>

<WithBorder>
	{#if post}
		<SinglePost {post} {replies} {likes} {reposts}>
			<slot />
		</SinglePost>
	{:else}
		<Posts {posts} />
	{/if}
</WithBorder>

<Footer />
