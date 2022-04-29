<script lang="ts">
	export let src: string | any[];
	export let alt: string = '';

	const allSources = Array.isArray(src) ? src : [{ srcset: src }];
	const sources = allSources.slice(0, -1);
	const lastSource = allSources[allSources.length - 1];
</script>

<picture {...$$restProps}>
	{#each sources as source}
		<source {...source} />
	{/each}
	<img {...lastSource} {alt} />
</picture>
