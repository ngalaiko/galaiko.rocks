<script lang="ts">
	import type { PageData } from './$types';
	import { addYears, formatRelative } from 'date-fns';

	export let data: PageData;
</script>

<svelte:head>
	<title>Restaurants and Cafes</title>
</svelte:head>

<article class="flex flex-col gap-2">
	<h1 class="text-2xl text-bold">Restaurants and Cafes</h1>
	<p>
		food establishments i went to the most since {formatRelative(
			addYears(new Date(), -1),
			new Date()
		)}:
	</p>
	<table class="table-auto w-full">
		<thead>
			<tr class="text-gray text-md font-semibold text-left">
				<th scope="col" class="pt-3 px-2">Place</th>
				<th scope="col" class="pt-3 px-2">Times</th>
				<th scope="col" class="pt-3 px-2">Spent</th>
			</tr>
			{#each data.places as { payee, count, amount, currency }}
				<tr>
					<td class="md:whitespace-nowrap px-2 md:px-3 py-4">{payee}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{count}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{amount.toLocaleString()} {currency}</td>
				</tr>
			{/each}
		</thead>
	</table>
</article>
