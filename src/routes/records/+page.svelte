<script lang="ts">
	import Image from '$lib/Image.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Records</title>
</svelte:head>

<article>
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
							<img src={info.coverImage} alt={fullName} lazy={true} />
						{/if}
						<figcaption class="text-sm text-center m-1">{fullName}</figcaption>
					</figure>
				</a>
			</li>
		{/each}
	</ul>
</article>
