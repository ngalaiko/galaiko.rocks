<script lang="ts">
	import type { ImageAttrs } from 'vite-plugin-image-presets';

	export let src: string | ImageAttrs[];
	export let alt: string = '';
	let className = '';
	export { className as class };

	const escapeSrc = (src: string) => (src ? encodeURI(src) : src);

	const escapeSrcset = (srcset: string) =>
		srcset
			?.split(', ')
			.map((s) => {
				const lastSpace = s.lastIndexOf(' ');
				return lastSpace === -1
					? escapeSrc(s)
					: escapeSrc(s.substring(0, lastSpace)) + ' ' + s.substring(lastSpace);
			})
			.join(', ');

	const allSources = Array.isArray(src)
		? src.map(({ srcset, src, ...rest }) => ({
				...rest,
				srcset: escapeSrcset(srcset),
				src: escapeSrc(src)
		  }))
		: [{ srcset: src }];

	const sources = allSources.slice(0, -1);
	const lastSource = allSources[allSources.length - 1];
</script>

<picture {...$$restProps} class={className}>
	{#each sources as source}
		<source {...source} />
	{/each}
	<img {...lastSource} {alt} />
</picture>
