<script lang="ts">
	import type { ImageAttrs } from 'vite-plugin-image-presets';

	export let src: string | ImageAttrs[];
	export let alt: string = '';
	let className = '';
	export { className as class };

	const escapeSrcset = (srcset: string) =>
		srcset
			?.split(', ')
			.map((s) => s.split(' '))
			.map((sliced) => `${sliced.slice(0, -1).join('%20')} ${sliced.slice(-1)[0]}`)
			.join(', ');

	const allSources = Array.isArray(src)
		? src.map(({ srcset, ...rest }) => ({
				...rest,
				srcset: escapeSrcset(srcset)
		  }))
		: [{ srcset: escapeSrcset(src) }];

	const sources = allSources.slice(0, -1);
	const lastSource = allSources[allSources.length - 1];
</script>

<picture {...$$restProps} class={className}>
	{#each sources as source}
		<source {...source} />
	{/each}
	<img {...lastSource} {alt} />
</picture>
