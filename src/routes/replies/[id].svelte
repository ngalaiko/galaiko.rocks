<script context="module" lang="ts">
	import type { Reply } from '$lib/webmentions';
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async ({ params, fetch }) => {
		const res = await fetch('/replies.json');
		if (!res.ok) return { status: 500, error: 'Something went wrong' };
		const replies = (await res.json()) as (Reply & { id: string })[];
		const reply = replies.find(({ id }) => id === params.id);
		return reply ? { status: 200, props: { reply } } : { status: 404 };
	};
</script>

<script lang="ts">
	import WithBorder from '$lib/components/layouts/WithBorder.svelte';
	import { Reply as ReplyComponent } from '$lib/components/molecules';

	export let reply: Reply;
</script>

<WithBorder>
	<ReplyComponent {reply} detailed />
</WithBorder>
