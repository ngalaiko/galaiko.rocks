<script lang="ts">
	import '../app.css';
</script>

<main id="main" class="mx-auto max-w-3xl min-h-screen flex flex-col justify-between">
	<div class="bg-background-soft text-lg pt-6 pb-12 mx-5 mb-32 md:px-20 md:pt-12 lg:mt-6">
		<slot />
	</div>

	<footer class="p-2 opacity-50 text-center">
		<a href="/">Nikita Galaiko</a>
		<span>&#183;</span>
		<a data-sveltekit-reload href="https://creativecommons.org/licenses/by-nc/4.0/" rel="license"
			>CC BY-NC 4.0</a
		>
		<span>&#183;</span>
		<span>2018...{new Date().getFullYear()}</span>
		<span>&#183;</span>
		<a rel="privacy" href="/privacy/">privacy policy</a>
		<span>&#183;</span>
		<a data-sveltekit-reload href="https://github.com/ngalaiko/galaiko.rocks">source</a>
	</footer>
</main>
