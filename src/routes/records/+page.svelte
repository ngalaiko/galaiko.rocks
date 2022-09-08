<script lang="ts">
	import Image from '$lib/Image.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Vinyl Records</title>
</svelte:head>

<article class="flex flex-col gap-2">
	<h1 class="text-2xl text-bold">Vinyl Records</h1>
	<p>here are all vinyl records that i have:</p>

	<ul class="grid grid-cols-2 sm:grid-cols-3 gap-3">
		{#each data.records as { artist, info, image }}
			{@const fullName = `${artist.name} - ${info.title}`}
			{@const discogsUrl = `https://www.discogs.com/release/${info.id}`}
			<li>
				<a href={discogsUrl}>
					<figure>
						{#if image}
							<Image src={image} alt={fullName} zoomable={false} />
						{:else}
							<img src={info.coverImage} alt={fullName} />
						{/if}
						<figcaption class="text-sm text-center m-1">{fullName}</figcaption>
					</figure>
				</a>
			</li>
		{/each}
	</ul>
</article>
