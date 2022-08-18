<script lang="ts">
	import '../app.css';
	import { WithBorder } from '$lib/components';
	import { Footer } from '$lib/components/molecules';
</script>

<main id="main" class="mx-auto max-w-3xl min-h-screen flex flex-col justify-between">
	<WithBorder>
		<slot />
	</WithBorder>
	<Footer />
</main>
