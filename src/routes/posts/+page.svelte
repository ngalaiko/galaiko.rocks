<script lang="ts">
	import { format } from 'date-fns';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Posts</title>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
</svelte:head>

<ul class="h-feed flex flex-col gap-2 text-lg">
	<h1 class="p-name mb-2 text-4xl font-bold">{data.posts.length} posts</h1>
	{#each data.posts as post, i}
		{@const day = post.date.getDay() + 1}
		{#if i == 0 || post.date.getFullYear() != data.posts[i - 1].date.getFullYear()}
			<time
				datetime={post.date.toISOString()}
				class="text-xl font-bold py-3 mt-6 flex justify-between"
			>
				<p>{format(post.date, 'yyyy')}</p>
				<p>{format(post.date, 'MMMM')}</p>
			</time>
		{:else if post.date.getMonth() != data.posts[i - 1].date.getMonth()}
			<time datetime={post.date.toISOString()} class="text-xl font-bold py-3 mt-6 flex justify-end">
				<p>{format(post.date, 'MMMM')}</p>
			</time>
		{/if}
		<li class="h-entry">
			<a data-sveltekit-prefetch class="u-url flex items-baseline gap-2" href={post.path}>
				<p class="p-name entry-title flex-initial text-ellipsis overflow-hidden whitespace-nowrap">
					{post.title}
				</p>
				<hr class="border-b-1 border-dotted flex-auto opacity-75" />
				<time class="dt-published flex-initial" datetime={post.date.toISOString()}>
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
