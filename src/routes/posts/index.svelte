<script lang="ts" context="module">
	export const load = async ({ fetch }) => {
		const url = '/posts.json';
		const res = await fetch(url);
		if (res.ok) {
			const posts = await res.json();
			return {
				props: { posts }
			};
		} else {
			return {
				status: 500,
				error: new Error(`Could not fetch ${url}`)
			};
		}
	};
</script>

<script lang="ts">
	import { format } from 'date-fns';
	import type { Post } from '$lib/posts';

	export let posts: Post[] = [];

	const data = posts.map((post) => ({
		date: new Date(post.date),
		post
	}));
</script>

<svelte:head>
	<title>Posts</title>
</svelte:head>

<ul class="h-feed flex flex-col gap-2 text-lg">
	<h1 class="p-name mb-2 text-4xl font-bold">{posts.length} posts</h1>
	{#each data as d, i}
		{@const { date, post } = d}
		{@const day = date.getDay() + 1}
		{#if i == 0 || date.getFullYear() != data[i - 1].date.getFullYear()}
			<time datetime={date.toISOString()} class="text-xl font-bold py-3 mt-6 flex justify-between">
				<p>{format(date, 'yyyy')}</p>
				<p>{format(date, 'MMMM')}</p>
			</time>
		{:else if date.getMonth() != data[i - 1].date.getMonth()}
			<time datetime={date.toISOString()} class="text-xl font-bold py-3 mt-6 flex justify-end">
				<p>{format(date, 'MMMM')}</p>
			</time>
		{/if}
		<li class="h-entry">
			<a class="u-url flex items-baseline gap-2" href={post.path}>
				<p class="p-name entry-title flex-initial text-ellipsis overflow-hidden whitespace-nowrap">
					{post.title}
				</p>
				<hr class="border-b-1 border-dotted flex-auto opacity-75" />
				<time class="dt-published flex-initial" datetime={date.toISOString()}>
					{#if day == 1 || day == 21 || day == 31}
						{day}st
					{:else if day == 2 || day == 22}
						{day}nd
					{:else if day == 3 || day == 23}
						{day}rd
					{:else}
						{day}th
					{/if}
				</time>
			</a>
		</li>
	{/each}
</ul>
