<script>
	import { format } from 'date-fns';

	/** @type {string} */
	export let title;
	/** @type {string} */
	export let date;
   
	const parsedDate = new Date(date);
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<h1 class="p-name mb-1 text-4xl font-bold">{title}</h1>
<div class="mb-8">
	<time class="dt-published" datetime={parsedDate.toISOString()}
		>{format(parsedDate, 'MMMM dd, yyyy')}</time
	>
</div>

<div class="e-content">
	<slot />
</div>
