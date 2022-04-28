<script lang="ts">
	import ArticleWithFooter from '../layouts/ArticleWithFooter.svelte';

	import { format } from 'date-fns';
	import type { Post } from '$lib/posts';
	import type { Reply, Like, Repost, Mention } from '$lib/webmentions';
	import Webmentions from './Webmentions.svelte';

	export let post: Post;
	export let likes: Like[];
	export let replies: Reply[];
	export let reposts: Repost[];
	export let mentions: Mention[];

	const parsedDate = new Date(post.date);
</script>

<svelte:head>
	<title>{post.title}</title>
</svelte:head>

<ArticleWithFooter>
	<h1 class="p-name mb-1 text-4xl font-bold">{post.title}</h1>
	<div class="mb-8">
		<time class="dt-published" datetime={parsedDate.toISOString()}
			>{format(parsedDate, 'MMMM dd, yyyy')}</time
		>
	</div>

	<div class="content e-content">
		<slot />
	</div>

	<div class="pt-4">
		<hr class="text-gray p-4" />
	</div>

	<Webmentions {replies} {likes} {reposts} {mentions} />
</ArticleWithFooter>
