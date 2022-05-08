<script lang="ts">
	import { format } from 'date-fns';
	import type { Post } from '$lib/posts';
	import type { Reply, Like, Repost } from '$lib/webmentions';
	import Webmentions from './Webmentions.svelte';

	export let post: Post;
	export let likes: Like[];
	export let replies: Reply[];
	export let reposts: Repost[];

	const parsedDate = new Date(post.date);
</script>

<svelte:head>
	<title>{post.title}</title>
	{#if post.previous}
		<link rel="prev" href={post.previous.path} />
	{/if}
	{#if post.next}
		<link rel="next" href={post.next.path} />
	{/if}
</svelte:head>

<article class="h-entry">
	<h1 class="p-name mb-1 text-4xl font-bold">{post.title}</h1>
	<div class="mb-8">
		<time class="dt-published" datetime={parsedDate.toISOString()}
			>{format(parsedDate, 'MMMM dd, yyyy')}</time
		>
	</div>

	<div class="content e-content">
		<slot />
	</div>

	<hr class="text-gray py-4" />

	<div class="flex justify-between pb-4">
		{#if post.previous}
			<a href={post.previous.path}>⇐ {post.previous.title}</a>
		{:else}
			<span />
		{/if}

		{#if post.next}
			<a href={post.next.path}>{post.next.title} ⇒</a>
		{:else}
			<span />
		{/if}
	</div>

	<Webmentions {replies} {likes} {reposts} />
</article>
