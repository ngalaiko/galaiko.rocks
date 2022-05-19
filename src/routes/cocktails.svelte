<script lang="ts">
	import { list, measurements, measurementToString } from '$lib/cocktails';
</script>

<svelte:head>
	<title>Cocktails</title>
</svelte:head>

<ul class="flex flex-col gap-2">
	{#each list as cocktail}
		<li>
			<h3 class="font-semibold">{cocktail.title}</h3>
			<ul>
				{#each measurements(cocktail) as measurement}
					<li>{measurementToString(measurement)}</li>
				{/each}
			</ul>
		</li>
	{/each}
</ul>
