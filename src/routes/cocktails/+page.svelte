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

<article>
	<h1>Cocktails</h1>
	<p>cocktails that i like and make:</p>

	<ul id="cocktails">
		{#each data.cocktails as { title, ingredients, steps, image, source }}
			{@const slug = slugify(title)}
			{@const parsedSource = parseSource(source)}
			<li id="cocktail">
				<a href={'#' + slug}>
					<h2 id={slug}>
						{title}
					</h2>
				</a>

				<div>
					<div>
						<ul>
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
							<small>
								source:
								{#if parsedSource instanceof URL}
									<a href={parsedSource.toString()}>{parsedSource.hostname}</a>
								{:else}
									{parsedSource}
								{/if}
							</small>
						{/if}
					</div>

					<Image src={image} alt={title} />
				</div>
			</li>
		{/each}
	</ul>
</article>

<style>
	#cocktails {
		list-style: none;
		padding: 0;
	}

	#cocktail > div {
		display: grid;
		gap: 0.5rem;
		grid-template-columns: repeat(3, minmax(0, 1fr));
	}

	#cocktail > div > div {
		grid-column: span 3 / span 3;
	}

	:global(#cocktail > div > picture) {
		grid-column: span 3 / span 3;
	}

	@media (min-width: 1024px) {
		#cocktail > div > div {
			grid-column: span 2 / span 2;
		}

		:global(#cocktail > div > picture) {
			grid-column: span 1 / span 1;
		}
	}

	small {
		color: var(--foreground-4);
	}
</style>
