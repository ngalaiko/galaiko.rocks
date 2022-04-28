<script lang="ts">
	import { format } from 'date-fns';
	import type { Mention } from '$lib/webmentions';

	export let mention: Mention;
</script>

<div class="p-like h-cite flex flex-col">
	<span>
		<a class="u-author h-card underline" href={mention.source}
			>{mention.author.name ?? new URL(mention.author.url).hostname}</a
		> liked this
	</span>
	<time class="dt-published opacity-50 text-sm" datetime={new Date(mention.timestamp).toISOString()}
		>{format(new Date(mention.timestamp), 'MMMM dd, yyyy')}</time
	>
</div>
