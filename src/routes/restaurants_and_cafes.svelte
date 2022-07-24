<script lang="ts" context="module">
	import { list, type Place } from '$lib/restaurants-and-cafes';
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async () => ({
		props: {
			places: list()
		}
	});
</script>

<script lang="ts">
	export let places: Place[];
</script>

<svelte:head>
	<title>Restaurants and Cafes</title>
</svelte:head>

<article>
	<table class="table-auto w-full">
		<thead>
			<tr class="text-gray text-md font-semibold text-left">
				<th scope="col" class="pt-3 px-2">Place</th>
				<th scope="col" class="pt-3 px-2">Times</th>
				<th scope="col" class="pt-3 px-2">Spent</th>
			</tr>
			{#each places as { payee, count, amount, currency }}
				<tr>
					<td class="md:whitespace-nowrap px-2 md:px-3 py-4">{payee}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{count}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{amount.toLocaleString()} {currency}</td>
				</tr>
			{/each}
		</thead>
	</table>
</article>
