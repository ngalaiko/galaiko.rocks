<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData, ActionData } from './$types';
	import { format } from 'date-fns';
	import { enhance } from '$app/forms';
	import { IconRss } from '$lib/assets/icons';

	export let data: PageData;
	export let form: ActionData;
</script>

<svelte:head>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
	<link rel="alternate" title="Comments" type="application/atom+xml" href="comments.atom" />
	<title>{data.post.title}</title>
	{#if data.post.previous}
		<link rel="prev" href={data.post.previous.path} />
	{/if}
	{#if data.post.next}
		<link rel="next" href={data.post.next.path} />
	{/if}

	<meta property="og:type" content="article" />
	<meta property="og:title" content={data.post.title} />
	<meta property="article:published_time" content={data.post.date.toISOString()} />
	<meta property="article:author" content="Nikita Galaiko" />
	<meta property="article:section" content={data.post.section} />
	{#each data.post.tags as tag}
		<meta property="article:tag" content={tag} />
	{/each}
</svelte:head>

<article class="h-entry">
	<header>
		<h1 class="p-name ">{data.post.title}</h1>
		<time class="dt-published" datetime={data.post.date.toISOString()}
			>{format(data.post.date, 'MMMM dd, yyyy')}</time
		>
	</header>

	<div class="e-content">
		<svelte:component this={data.post.default} />
	</div>
</article>

<section id="navigation">
	{#if data.post.previous}
		<a id="previous" data-sveltekit-reload href={data.post.previous.path}
			>⇐ {data.post.previous.title}</a
		>
	{:else}
		<span />
	{/if}

	{#if data.post.next}
		<a id="next" data-sveltekit-reload href={data.post.next.path}>{data.post.next.title} ⇒</a>
	{:else}
		<span />
	{/if}
</section>

<section id="comments">
	<header>
		<h3>Comments:</h3>
		<a data-sveltekit-reload href="comments.atom">
			<IconRss />
		</a>
	</header>
	<ul>
		{#each data.comments as comment}
			<li>
				<article class="comment">
					<header>
						<strong>{comment.authorName}</strong>
						<time datetime={comment.created.toISOString()}
							>{format(comment.created, 'MMMM dd, yyyy')}</time
						>
					</header>
					<div>
						<svelte:component this={comment.default} />
					</div>
				</article>
			</li>
		{:else}
			<p>No comments yet</p>
		{/each}
	</ul>

	{#if form && form.message && form.success}
		<p>{form.message}</p>
	{:else}
		<form method="POST" use:enhance>
			<input name="pathname" value={$page.url.pathname} type="text" hidden />
			<input name="challange" value={data.challange} type="text" hidden />

			<div id="author">
				<label for="author_name"> Your name: </label>
				<input name="author_name" type="text" required />
			</div>

			<textarea rows="3" name="body" type="text" required />

			<div id="solution">
				<div>
					<label for="solution">{data.challange} = </label>
					<input name="solution" type="text" required />
				</div>

				<button type="submit"> Comment </button>
			</div>

			{#if form && form.message}
				<p>{form.message}</p>
			{/if}
		</form>
	{/if}
</section>

<style>
	section#comments {
		margin-top: 3rem;
	}

	section#comments > header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	section#comments > ul {
		padding: 0;
		list-style-type: none;
	}

	#navigation {
		margin-top: 3rem;
		padding-top: 1rem;
		border-top: 1px solid var(--foreground);

		display: flex;
		justify-content: space-between;

		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	#navigation #next {
		padding-left: 1rem;
	}

	#navigation #previous {
		padding-right: 1rem;
	}

	#solution {
		display: flex;
		justify-content: space-between;
	}

	#solution input {
		width: 2em;
	}

	article header h1 {
		margin-bottom: 0.5rem;
	}

	article header {
		margin-bottom: 1em;
	}

	button,
	input,
	textarea {
		font-size: inherit;
		background: inherit;
		color: inherit;
		border: 2px solid var(--foreground);
		margin: 0 5px;
	}

	button {
		border: 0;
		cursor: pointer;
		text-align: right;
		text-decoration: underline;
	}

	form {
		display: grid;
		gap: 1rem;
	}

	.comment {
		margin-bottom: 2rem;
	}

	.comment div {
		margin-top: -1rem;
	}

	.comment header {
		display: flex;
		justify-content: space-between;
	}
</style>
