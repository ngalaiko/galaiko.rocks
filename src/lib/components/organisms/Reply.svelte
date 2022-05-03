<script lang="ts">
	import { format } from 'date-fns';
	import type { Reply } from '$lib/webmentions';

	export let reply: Reply;
</script>

<div class="u-comment h-cite flex-col">
	<a class="u-author h-card underline" href={reply.source}
		>{reply.author.name ?? new URL(reply.author.url).hostname}:</a
	>
	<p class="p-content p-name pl-2">{reply.content}</p>
	{#if reply.updated}
		<time class="dt-published opacity-50 text-sm" datetime={new Date(reply.updated).toISOString()}
			>{format(new Date(reply.updated), 'MMMM dd, yyyy')}</time
		>
	{:else}
		<time class="dt-published opacity-50 text-sm" datetime={new Date(reply.published).toISOString()}
			>{format(new Date(reply.published), 'MMMM dd, yyyy')}</time
		>
	{/if}
</div>
