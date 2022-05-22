<script lang="ts">
	import list from '$lib/cocktails';
</script>

<svelte:head>
	<title>Cocktails</title>
</svelte:head>

<ul class="flex flex-col gap-2">
	{list.map((r) => r.steps)}
</ul>
