<script lang="ts">
	import { Article } from '$lib/components';
	import restaurantsAndCafes from '$lib/data/restaurants_and_cafes.json';
</script>

<Article>
	<table class="table-auto w-full">
		<thead>
			<tr class="text-gray text-md font-semibold text-left">
				<th scope="col" />
				<th scope="col" class="py-3 px-2">Place</th>
				<th scope="col" class="py-3 px-2">Times</th>
				<th scope="col" class="py-3 px-2">Spent</th>
			</tr>
			{#each restaurantsAndCafes as { payee, count, amount, currency }, index}
				<tr>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{index + 1}</td>
					<td class="md:whitespace-nowrap px-2 md:px-3 py-4">{payee}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{count}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{amount.toLocaleString()} {currency}</td>
				</tr>
			{/each}
		</thead>
	</table>
</Article>
