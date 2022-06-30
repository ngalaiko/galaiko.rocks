<script lang="ts">
	import { list } from '$lib/records';
</script>

<svelte:head>
	<title>Records</title>
</svelte:head>

<article>
	<ul class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-1">
		{#each list() as { artist, info }}
			{@const fullName = `${artist.name} ${info.title}`}
			{@const discogsUrl = `https://www.discogs.com/release/${info.id}`}
			{#if info.coverImage}
				<li>
					<a href={discogsUrl}>
						<img src={info.coverImage} alt={fullName} lazy={true} />
					</a>
				</li>
			{/if}
		{/each}
	</ul>
</article>
