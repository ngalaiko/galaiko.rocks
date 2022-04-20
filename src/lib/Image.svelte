<script lang="ts">
	// This is a copy of https://github.com/matyunya/svelte-image/blob/master/src/Image.svelte
	// but with the following changes:
	// - urls in srcsets are always absolute
	// - blurhash is removed
	// - Waypoint is removed
	export let alt = '';
	export let width = null;
	export let height = null;
	export let usemap = '';
	export let src = '';
	export let srcset = '';
	export let srcsetWebp = '';
	export let ratio = '100%';
	export let blur = true;
	export let sizes = '(max-width: 1000px) 100vw, 1000px';
	export let placeholderClass = '';

	const rootPath = (srcset) =>
		srcset
			.split(',')
			.map((src) => {
				const [url, size] = src.split(' ');
				return `/${url} ${size}`;
			})
			.join(',');

	srcset = rootPath(srcset);
	srcsetWebp = rootPath(srcsetWebp);

	let className = '';
	export { className as class };

	let loaded = true;
</script>

<div class:loaded style="position: relative; width: 100%;">
	<div style="position: relative; overflow: hidden;">
		<div style="width:100%;padding-bottom:{ratio};" />
		<img class="placeholder {placeholderClass}" class:blur {src} {alt} />
		<picture>
			<source type="image/webp" srcset={srcsetWebp} {sizes} />
			<source {srcset} {sizes} />
			<img {src} class="main {className}" {alt} {width} {height} {usemap} />
		</picture>
	</div>
</div>

<style>
	img {
		object-position: center;
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		will-change: opacity;
	}

	.blur {
		filter: blur(15px);
		transition: opacity 1200ms;
	}

	.placeholder {
		opacity: 1;
		width: 100%;
		height: 100%;
		transition: opacity 1200ms ease-out;
		transition-delay: 0.4s;
	}

	.main {
		opacity: 0;
		transition: opacity 1200ms ease-out;
		transition-delay: 0.4s;
	}

	.loaded .placeholder {
		opacity: 0;
	}

	.loaded .main {
		opacity: 1;
	}
</style>
