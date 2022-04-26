<script lang="ts">
	import type { Reply, Like } from '$lib/webmentions';
	import ReplyComponent from './Reply.svelte';
	import LikeComponent from './Like.svelte';

	export let replies: Reply[];
	export let likes: Like[];

	const mentions = replies
		.map(
			(reply) =>
				['reply', reply, new Date(reply.updated) ?? new Date(reply.published)] as [
					string,
					Reply,
					Date
				]
		)
		.concat(likes.map((like) => ['like', like, new Date(like.timestamp)] as [string, Like, Date]))
		.sort((a, b) => b[2].getTime() - a[2].getTime());
</script>

<div>
	<h2 class="text-xl">{mentions.length} webmentions{mentions.length ? ':' : ''}</h2>
	<ul class="flex flex-col gap-2">
		{#each mentions as [type, data]}
			<li class="py-2">
				{#if type === 'like'}
					<LikeComponent like={data} />
				{:else if type === 'reply'}
					<ReplyComponent reply={data} />
				{/if}
			</li>
		{:else}
			<li class="py-2">
				<a sveltekit:reload class="text-blue" href="https://www.w3.org/TR/webmention/">Send one!</a>
			</li>
		{/each}
	</ul>
</div>
