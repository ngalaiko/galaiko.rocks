<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';
	import type { Cocktail } from '$lib/cocktails';

	export const load: Load = async ({ fetch }) => {
		const res = await fetch('/cocktails.json');
		if (!res.ok) return { status: 500, error: 'Something went wrong' };
		const cocktails = (await res.json()) as Cocktail[];
		return { status: 200, props: { cocktails } };
	};
</script>

<script lang="ts">
	export let cocktails: Cocktail[];

	const slugify = (s: string) => s.toLowerCase().replace(/\s+/g, '-');
</script>

<svelte:head>
	<title>Cocktails</title>
</svelte:head>
<ul class="flex flex-col gap-6">
	{#each cocktails as cocktail}
		{@const slug = slugify(cocktail.title)}
		<li class="flex flex-col gap-2">
			<h2 id={slug}>
				<a href={'#' + slug} class="font-semibold underline">{cocktail.title}</a>
			</h2>
			<ul class="list-disc ml-5 ">
				{#each cocktail.ingredients as ingredient}
					<li>
						<span>{ingredient.name}{ingredient.quantity ? ':' : ''}</span>
						{#if ingredient.quantity}
							<span>{ingredient.quantity}</span>
						{/if}
					</li>
				{/each}
			</ul>
			<p>{cocktail.steps.join(' ')}</p>
		</li>
	{/each}
</ul>
