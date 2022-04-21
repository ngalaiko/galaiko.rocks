<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';

	export const load: Load = ({ status }) => {
		return {
			props: {
				title: status === 404 ? 'Page not found.' : 'Something went wrong.'
			}
		};
	};
</script>

<script lang="ts">
	export let title: string;
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<h1 class="w-full h-[50vh] flex items-center justify-center">{title}</h1>
