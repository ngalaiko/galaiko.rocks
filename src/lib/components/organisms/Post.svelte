<script lang="ts">
	import ArticleWithFooter from '../layouts/ArticleWithFooter.svelte';

	import { format } from 'date-fns';
	import type { Post } from '$lib/posts';

	export let post: Post;

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
</ArticleWithFooter>
