<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData, ActionData } from './$types';
	import { format } from 'date-fns';
	import { enhance } from '$app/forms';

	export let data: PageData;
	export let form: ActionData;
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

<div class="flex flex-col gap-6">
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
	</article>

	<hr class="text-gray" />

	<div class="flex justify-between  text-sm">
		{#if data.post.previous}
			<a data-sveltekit-reload href={data.post.previous.path}>⇐ {data.post.previous.title}</a>
		{:else}
			<span />
		{/if}

		{#if data.post.next}
			<a data-sveltekit-reload href={data.post.next.path}>{data.post.next.title} ⇒</a>
		{:else}
			<span />
		{/if}
	</div>

	{#if form && form.message && form.success}
		<p>{form.message}</p>
	{:else}
		<form method="POST" class="flex flex-col gap-2" use:enhance>
			<div class="flex gap-2 items-ceter justify-between">
				<div class="flex items-center whitespace-nowrap">
					<label for="author_name"> Name: </label>
					<input
						name="author_name"
						type="text"
						class="bg-background-soft border-b-2 p-1 focus:outline-none"
						required
					/>
				</div>

				<div class="flex gap-2 items-center whitespace-nowrap">
					<label for="solution">{data.challange} = </label>
					<input
						name="solution"
						type="text"
						class="focus:outline-none w-[2em] bg-background-soft border-b-2 p-1"
						required
					/>
				</div>
			</div>

			<textarea
				rows="3"
				class="focus:outline-none bg-background-soft border-2 p-1"
				name="body"
				type="text"
				required
			/>

			<input name="pathname" value={$page.url.pathname} type="text" hidden />
			<input name="challange" value={data.challange} type="text" hidden />

			<div class="flex flex-row-reverse justify-between">
				<input
					type="submit"
					class="cursor-pointer underline hover:text-foreground-1"
					value="Comment"
				/>
				{#if form && form.message}
					<p class="text-red">{form.message}</p>
				{/if}
			</div>
		</form>
	{/if}
</div>
