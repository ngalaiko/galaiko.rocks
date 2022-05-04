<script lang="ts">
	import type { Reply, Like, Repost } from '$lib/webmentions';
	import {
		Reply as ReplyComponent,
		Like as LikeComponent,
		Repost as RepostComponent
	} from '$lib/components/molecules';

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

<section>
	<h2 class="text-xl">{all.length} mentions{all.length ? ':' : ''}</h2>
	{#each all as { component, props }}
		<svelte:component this={component} {...props} />
	{:else}
		<a sveltekit:reload class="text-blue" href="https://indieweb.org/Webmention">Send one!</a>
	{/each}
</section>
