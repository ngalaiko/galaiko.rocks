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
		if (res.ok) {
			const body = (await res.json()) as Post | Post[];
			return body instanceof Array
				? { status: 200, props: { posts: body } }
				: body.path === url.pathname
				? { status: 200, props: { post: body } }
				: { status: 301, redirect: body.path };
		} else {
			return res.status === 404
				? { status: 404, error: 'Page not found' }
				: { status: 500, error: 'Something went wrong' };
		}
	};
</script>

<script lang="ts">
	import { Post as SinglePost, Posts } from '$lib/components';

	export let post: Post;
	export let posts: Post[];
</script>

{#if post}
	<SinglePost {post}>
		<slot />
	</SinglePost>
{:else}
	<Posts {posts} />
{/if}
