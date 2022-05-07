<script lang="ts">
	import { format } from 'date-fns';
	import type { Repost } from '$lib/webmentions';

	export let repost: Repost;
</script>

<article class="h-entry p-like flex flex-col p-2">
	<span>
		<a class="u-author h-card underline" href={repost.source}
			>{repost.author.name ?? new URL(repost.author.url).hostname}</a
		> reposted this
	</span>
	{#if repost.timestamp}
		<time
			class="dt-published opacity-50 text-sm"
			datetime={new Date(repost.timestamp).toISOString()}
			>{format(new Date(repost.timestamp), 'MMMM dd, yyyy')}</time
		>
	{/if}
</article>
