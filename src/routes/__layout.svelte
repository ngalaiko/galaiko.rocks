<script>
	import '../app.css';
	import { ColorToggle, WithBorder } from '$lib/components';
	import { Footer } from '$lib/components/molecules';
</script>

<ColorToggle />

<main id="main" class="mx-auto max-w-3xl min-h-screen flex flex-col justify-between">
	<WithBorder>
		<slot />
	</WithBorder>
	<Footer />
</main>
