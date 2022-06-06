<script>
	import '../app.css';
	import { ColorToggle, WithBorder } from '$lib/components';
	import { Footer } from '$lib/components/molecules';
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

<ColorToggle />

<main id="main" class="mx-auto max-w-3xl min-h-screen flex flex-col justify-between">
	<WithBorder>
		<slot />
	</WithBorder>
	<Footer />
</main>
