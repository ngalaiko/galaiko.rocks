<script lang="ts">
	import type { ImageAttrs } from 'vite-plugin-image-presets';
	import ZoomableImage from './ZoomableImage.svelte';

	export let src: string | ImageAttrs[];
	export let alt = '';
	export let zoomable = true;
	let className = '';
	export { className as class };

	const allSources = Array.isArray(src) ? src : [{ srcset: src }];
	const sources = allSources.slice(0, -1);
	const lastSource = allSources[allSources.length - 1];
</script>

<picture class={className}>
	{#each sources as source}
		<source {...source} />
	{/each}
	{#if zoomable}
		<ZoomableImage attrs={lastSource} {alt} />
	{:else}
		<img {...lastSource} {alt} />
	{/if}
</picture>
