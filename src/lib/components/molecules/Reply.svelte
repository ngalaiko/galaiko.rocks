<script lang="ts">
	import { format } from 'date-fns';
	import type { Reply } from '$lib/webmentions';

	export let reply: Reply;
	export let detailed = false;
</script>

<article class="p-comment h-cite flex-col ml-[50px] mt-2">
	<div class="p-author h-card flex gap-2 font-small">
		{#if reply.author.picture}
			<img class="u-photo w-[50px] h-[50px] -ml-[50px]" src={reply.author.picture} alt="" />
		{:else}
			<div class="w-[50px] h-[50px] -ml-[50px]" />
		{/if}
		<a class="p-name u-url underline" href={reply.source}>
			{reply.author.name ?? new URL(reply.author.url).hostname}</a
		>
	</div>
	<p class="e-content ml-2 -mt-6">{reply.content}</p>
	{#if reply.updated}
		<time
			class="dt-published opacity-50 text-sm ml-2"
			datetime={new Date(reply.updated).toISOString()}
			>{format(new Date(reply.updated), 'MMMM dd, yyyy')}</time
		>
	{:else if reply.timestamp}
		<time
			class="dt-published opacity-50 text-sm ml-2"
			datetime={new Date(reply.timestamp).toISOString()}
			>{format(new Date(reply.timestamp), 'MMMM dd, yyyy')}</time
		>
	{/if}
	{#if detailed}
		<span class="opacity-50 text-sm"
			>in reply to <a class="u-in-reply-to underline" href={reply.target}>{reply.target}</a></span
		>
	{/if}
</article>
