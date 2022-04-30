<script lang="ts">
	import type { Reply, Like, Repost } from '$lib/webmentions';
	import ReplyComponent from './Reply.svelte';
	import LikeComponent from './Like.svelte';
	import RepostComponent from './Repost.svelte';

	export let replies: Reply[];
	export let likes: Like[];
	export let reposts: Repost[];

	const all = [
		...replies.map((reply) => ({
			component: ReplyComponent,
			props: { reply },
			sort: new Date(reply.updated) ?? new Date(reply.timestamp)
		})),
		...likes.map((like) => ({
			component: LikeComponent,
			props: { like },
			sort: new Date(like.timestamp)
		})),
		...reposts.map((repost) => ({
			component: RepostComponent,
			props: { repost },
			sort: new Date(repost.timestamp)
		}))
	].sort((a, b) => b.sort.getTime() - a.sort.getTime());
</script>

<div>
	<h2 class="text-xl">{all.length} mentions{all.length ? ':' : ''}</h2>
	<ul class="flex flex-col gap-2">
		{#each all as { component, props }}
			<li class="py-2">
				<svelte:component this={component} {...props} />
			</li>
		{:else}
			<li class="py-2">
				<a sveltekit:reload class="text-blue" href="https://indieweb.org/Webmention">Send one!</a>
			</li>
		{/each}
	</ul>
</div>
