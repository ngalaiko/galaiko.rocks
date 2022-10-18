<script lang="ts">
	import { format } from 'date-fns';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Posts</title>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
</svelte:head>

<article>
	<h1 class="p-name">{data.posts.length} posts</h1>

	<ul class="h-feed">
		{#each data.posts as post, i}
			{@const day = post.date.getDay() + 1}
			{#if i == 0 || post.date.getFullYear() != data.posts[i - 1].date.getFullYear()}
				<h2>
					<time datetime={post.date.toISOString()}>
						<span>{format(post.date, 'yyyy')}</span>
						<span>{format(post.date, 'MMMM')}</span>
					</time>
				</h2>
			{:else if post.date.getMonth() != data.posts[i - 1].date.getMonth()}
				<h2>
					<time datetime={post.date.toISOString()}>
						<span>{format(post.date, 'MMMM')}</span>
					</time>
				</h2>
			{/if}
			<li class="h-entry">
				<a data-sveltekit-prefetch class="u-url" href={post.path}>
					<span class="p-name">
						{post.title}
					</span>
					<hr />
					<time class="dt-published" datetime={post.date.toISOString()}>
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
</article>

<style>
	a {
		color: inherit;
		display: flex;
		padding: 0.5rem 0;
	}

	a:visited {
		color: inherit;
	}

	time {
		display: flex;
		justify-content: space-between;
	}

	ul {
		padding: 0;
		list-style-type: none;
	}

	hr {
		border: none;
		flex: auto;
		align-self: flex-end;
		margin: 0 0.3rem 0.3rem;
		border-top: 1px dotted;
	}

	h2 {
		text-align: justify;
	}
</style>
