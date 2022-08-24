<script lang="ts">
	import '../app.css';
	import { WithBorder } from '$lib/components';
	import { Footer } from '$lib/components/molecules';
	import { webVitals } from '$lib/vitals';
	import { browser, dev } from '$app/env';
	import { page } from '$app/stores';

	const analyticsId = import.meta.env.VERCEL_ANALYTICS_ID;
	$: if (!dev && browser && analyticsId) {
		webVitals({
			path: $page.url.pathname,
			params: $page.params,
			analyticsId
		});
	}
</script>

<main id="main" class="mx-auto max-w-3xl min-h-screen flex flex-col justify-between">
	<WithBorder>
		<slot />
	</WithBorder>
	<Footer />
</main>
