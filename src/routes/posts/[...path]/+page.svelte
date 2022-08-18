<script lang="ts">
	import type { PageData } from './$types';
	import { format } from 'date-fns';

	export let data: PageData;
</script>

<svelte:head>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
	<title>{data.post.title}</title>
	{#if data.post.previous}
		<link rel="prev" href={data.post.previous.path} />
	{/if}
	{#if data.post.next}
		<link rel="next" href={data.post.next.path} />
	{/if}
</svelte:head>

<article class="h-entry">
	<h1 class="p-name mb-1 text-4xl font-bold">{data.post.title}</h1>
	<div class="mb-8">
		<time class="dt-published" datetime={data.post.date.toISOString()}
			>{format(data.post.date, 'MMMM dd, yyyy')}</time
		>
	</div>

	<div class="content e-content">
		<svelte:component this={data.post.default} />
	</div>

	<hr class="text-gray py-4" />

	<div class="flex justify-between pb-4 text-sm">
		{#if data.post.previous}
			<a sveltekit:reload href={data.post.previous.path}>⇐ {data.post.previous.title}</a>
		{:else}
			<span />
		{/if}

		{#if data.post.next}
			<a sveltekit:reload href={data.post.next.path}>{data.post.next.title} ⇒</a>
		{:else}
			<span />
		{/if}
	</div>
</article>
