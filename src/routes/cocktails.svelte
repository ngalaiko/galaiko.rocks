<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import { type Cocktail, list } from '$lib/cocktails';

	export const load: Load = async () => {
		const cocktails = await list();
		return { status: 200, props: { cocktails } };
	};
</script>

<script lang="ts">
	import Image from '$lib/Image.svelte';

	export let cocktails: Cocktail[];

	const slugify = (s: string) => s.toLowerCase().replace(/\s+/g, '-');
</script>

<svelte:head>
	<title>Cocktails</title>
</svelte:head>
<div class="grid grid-cols-3 gap-x-4">
	{#each cocktails as { title, ingredients, steps, image, source }, i}
		{@const slug = slugify(title)}
		<h2 id={slug} class="col-span-3 sm:col-span-2 mb-4" class:mt-6={i !== 0}>
			<a href={'#' + slug} class="font-semibold underline">{title}</a>
		</h2>
		<div class="flex flex-col gap-2 col-span-3 sm:col-span-2">
			<ul class="list-disc ml-5 ">
				{#each ingredients as { name, quantity }}
					<li>
						<span>{name}{quantity ? ':' : ''}</span>
						{#if quantity}
							<span>{quantity}</span>
						{/if}
					</li>
				{/each}
			</ul>
			<p>{steps.join(' ')}</p>
			{#if source}
				<small class="text-foreground-4 -mt-1.5">
					source: <a class="underline" href={source.toString()}>{source.hostname}</a></small
				>
			{/if}
		</div>
		<Image src={image} alt={title} class="col-span-3 sm:col-span-1 my-4"/>
	{/each}
</div>
