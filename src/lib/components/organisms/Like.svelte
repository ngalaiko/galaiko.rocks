<script lang="ts">
	import { format } from 'date-fns';
	import type { Like } from '$lib/webmentions';

	export let like: Like;
</script>

<div class="p-like h-cite flex flex-col">
	<span>
		<a class="u-author h-card underline" href={like.source}
			>{like.author.name ?? new URL(like.author.url).hostname}</a
		> liked this
	</span>
	<time class="dt-published opacity-50 text-sm" datetime={new Date(like.timestamp).toISOString()}
		>{format(new Date(like.timestamp), 'MMMM dd, yyyy')}</time
	>
</div>
