<script lang="ts">
	import Image from '$lib/Image.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const slugify = (s: string) => s.toLowerCase().replace(/\s+/g, '-');
	const parseSource = (s?: string) => {
		if (!s) return null;
		try {
			return new URL(s);
		} catch {
			return s;
		}
	};
</script>

<svelte:head>
	<title>Cocktails</title>
</svelte:head>

<div class="grid grid-cols-3 gap-x-4">
	{#each data.cocktails as { title, ingredients, steps, image, source }, i}
		{@const slug = slugify(title)}
		{@const parsedSource = parseSource(source)}
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
					source:
					{#if parsedSource instanceof URL}
						<a class="underline" href={parsedSource.toString()}>{parsedSource.hostname}</a>
					{:else}
						{parsedSource}
					{/if}
				</small>
			{/if}
		</div>
		<Image src={image} alt={title} class="col-span-3 sm:col-span-1 my-4" />
	{/each}
</div>
