<script lang="ts">
	import { format } from 'date-fns';
	import type { Reply } from '$lib/webmentions';

	export let reply: Reply;
</script>

<div class="u-comment h-cite flex-col">
	<a class="u-author h-card underline" href={reply.author.url}>{reply.author.name}:</a>
	<div class="pl-2">
		<p class="p-content p-name">{reply.content}</p>
		{#if reply.published}
			<time
				class="dt-published opacity-50 text-sm"
				datetime={new Date(reply.published).toISOString()}
				>{format(new Date(reply.published), 'MMMM dd, yyyy')}</time
			>
		{/if}
	</div>
</div>
