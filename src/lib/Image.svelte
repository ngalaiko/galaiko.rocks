<script lang="ts">
	import type { ImageAttrs } from 'vite-plugin-image-presets';

	export let src: string | ImageAttrs[];
	export let alt: string = '';
	let className = '';
	export { className as class };

	const allSources = Array.isArray(src) ? src : [{ srcset: src }];
	const sources = allSources.slice(0, -1);
	const lastSource = allSources[allSources.length - 1];

	import Lightense from 'lightense-images';
	import { onMount } from 'svelte';
	onMount(() => {
		Lightense(document.querySelectorAll('.zoomable'), {
			time: 100,
			keyboard: true,
			cubicBezier: 'cubic-bezier(.2, 0, .1, 1)',
			background: 'var(--background)',
			zIndex: 1e6
		});
	});
</script>

<picture {...$$restProps} class={className}>
	{#each sources as source}
		<source {...source} />
	{/each}
	<img {...lastSource} {alt} />
</picture>
