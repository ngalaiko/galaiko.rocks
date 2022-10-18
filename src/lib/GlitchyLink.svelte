<script lang="ts" context="module">
	const angleIncrement = (0.5 * Math.PI) / 128;
	const TWO_PI = Math.PI * 2;
	const offsetBase = 1.55;

	let angle = 0;
	const beforeFrames = [];
	const afterFrames = [];

	for (let i = 0; i < 512; i++) {
		angle += angleIncrement;
		angle %= TWO_PI;
		const x = offsetBase * Math.sin(angle);
		const y = offsetBase * Math.cos(angle);
		beforeFrames.unshift({ transform: `translate(${x}px, ${y}px)` });
		afterFrames.unshift({ transform: `translate(${-1 * x}px, ${-1 * y}px)` });
	}

	const duration = 6000;
	const iterations = Infinity;
</script>

<script lang="ts">
	import { onMount } from 'svelte';

	export let href: string;

	let link: HTMLElement;
	onMount(() => {
		link.dataset['text'] = link.innerText;
		link.animate(beforeFrames, {
			duration,
			pseudoElement: ':before',
			iterations
		});
		link.animate(afterFrames, {
			duration,
			pseudoElement: ':after',
			iterations
		});
	});
</script>

<a {href} bind:this={link} {...$$restProps}>
	<slot />
</a>

<style>
	a {
		display: inline-block;
		color: var(--blue);
		position: relative;
		transition: color 0.5s linear 0.2s;
	}

	a:visited {
		color: var(--blue-dim);
	}

	a:after,
	a:before {
		transition: transform 0.5s ease-out, opacity 0.5s linear;
		opacity: 0;
		content: attr(data-text);
		position: absolute;
		top: 0;
		left: 0;
	}

	a:hover:after,
	a:hover:before {
		opacity: 0.75;
	}

	a:before {
		color: var(--red);
	}

	a:before:visited {
		color: var(--red-dim);
	}

	a:focus,
	a:hover {
		color: var(--green);
	}

	a:focus:visited,
	a:hover:visited {
		color: var(--green-dim);
	}

	a:after {
		color: var(--blue);
	}

	a:after:visited {
		color: var(--blue-dim);
	}
</style>
