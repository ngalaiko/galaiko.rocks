<script lang="ts">
	import '../app.css';
	import { inject } from '@vercel/analytics';
	import { page } from '$app/stores';
	import { dev } from '$app/environment';
	import { onMount } from 'svelte';

	$: ogImageUrl =
		$page.url.pathname === '/' ? '/index.png' : $page.url.pathname.slice(0, -1) + '.png';
	if (!dev) onMount(inject);
</script>

<svelte:head>
	<meta property="og:url" content={$page.url.toString()} />
	<meta property="og:image" content={ogImageUrl} />
	<meta property="og:locale" content="en_US" />
	<meta property="og:site_name" content="Nikita Galaiko's personal website" />
</svelte:head>

<main>
	<slot />
</main>

<footer>
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

<style>
	footer {
		font-size: inherit;
		line-height: inherit;

		padding-top: 11rem;
		text-align: center;
		color: var(--foreground-4);
	}

	footer > a {
		color: inherit;
	}
</style>
