<script>
	import cloudflareAnalytics from '$lib/assets/cloudflare-analytics.jpg?preset=hd';
	import Image from '$lib/Image.svelte';
</script>

<svelte:head>
	<title>Privacy Policy</title>
</svelte:head>

<arcicle class="content">
	<h1>Privacy Policy</h1>

	<h2>Hosting</h2>
	<p>
		First thing to know, this website is hosted on <a
			data-sveltekit-reload
			href="https://pages.cloudflare.com/">Cloudflare Pages</a
		>. It means that all your requests go through their servers and
		<a data-sveltekit-reload href="https://www.cloudflare.com/en-gb/privacypolicy/"
			>Cloudflare's Privacy policy</a
		> is applied.
	</p>

	<h2>JavaScript</h2>
	<p>
		This website is build with <a data-sveltekit-reload href="https://kit.svelte.dev">SvelteKit</a> and by
		default uses JavaScript to act like a Single Page Application for enhanced user expirience.
	</p>

	<p>
		Also, Cloudflare is injecting a small amount of JavaScript that allows me to see some web
		analytics.
	</p>

	<p>
		With that said, the website is fully functional without JavaScript if you don't want to use it.
	</p>

	<h2>Analytics</h2>
	<p>
		I use <a data-sveltekit-reload href="https://www.cloudflare.com/en-gb/web-analytics/"
			>Cloudflare's Web Analytics</a
		>
		to track some basic information about the visitors of the website.
	</p>

	<p>This is what it roughly looks like for me:</p>

	<Image src={cloudflareAnalytics} alt="Cloudflare Analytics" />

	<h2>Cookies</h2>
	<p>No cookies usage.</p>

	<h2>Source</h2>
	<p>
		You can find the website's source code on <a
			data-sveltekit-reload
			href="https://github.com/ngalaiko/galaiko.rocks">GitHub</a
		>.
	</p></arcicle
>
