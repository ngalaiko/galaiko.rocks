<script context="module" lang="ts">
	import type { Reply } from '$lib/webmentions';
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async ({ fetch }) => {
		const res = await fetch('/replies.json');
		const replies = (await res.json()) as Reply[];
		return res.ok
			? { status: 200, props: { replies } }
			: { status: 500, error: 'Something went wrong' };
	};
</script>

<script lang="ts">
	import { Reply as ReplyComponent } from '$lib/components/molecules';

	export let replies: Reply[] = [];
</script>

{#each replies as reply}
	<ReplyComponent {reply} detailed />
{/each}
