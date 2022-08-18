<script lang="ts">
	import type { ImageAttrs } from 'vite-plugin-image-presets';
	import ZoomableImage from './ZoomableImage.svelte';

	export let src: string | ImageAttrs[];
	export let alt = '';
	export let zoomable = true;
	let className = '';
	export { className as class };

	// for some reason, src becomes relative when ran in the browser. possibly it has
    // to do something with browser vs node
	// nevertheless, this is a hack to make src absolute again
	const ensureAbsoluteSrc = (src: string) => {
		const index = src.indexOf('/assets/');
		if (index === 0) {
			return src;
		}
		return src.substring(index);
	};
	const allSources = (Array.isArray(src) ? src : [{ srcset: src }]).map(
		({ srcset, src, ...rest }) => ({
			...rest,
			src: src ? ensureAbsoluteSrc(src) : src,
			srcset: srcset.split(', ').map(ensureAbsoluteSrc).join(', ')
		})
	);

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
