<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData, ActionData } from './$types';
	import { format } from 'date-fns';
	import { enhance } from '$app/forms';

	export let data: PageData;
	export let form: ActionData;
</script>

<svelte:head>
	<link rel="alternate" title="All Posts" type="application/atom+xml" href="/posts.atom" />
	<title>{data.post.title}</title>
	{#if data.post.previous}
		<link rel="prev" href={data.post.previous.path} />
	{/if}
	{#if data.post.next}
		<link rel="next" href={data.post.next.path} />
	{/if}
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
		<a data-sveltekit-reload href={data.post.previous.path}>⇐ {data.post.previous.title}</a>
	{:else}
		<span />
	{/if}

	{#if data.post.next}
		<a data-sveltekit-reload href={data.post.next.path}>{data.post.next.title} ⇒</a>
	{:else}
		<span />
	{/if}
</section>

<section id="comments">
	<h3>Comments:</h3>
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
	#comments {
		margin-top: 3rem;
	}

	#comments > ul {
		padding: 0;
		list-style-type: none;
	}

	#navigation {
		margin-top: 3rem;
		padding-top: 1rem;
		border-top: 1px solid var(--foreground);
	}

	#navigation {
		display: flex;
		justify-content: space-between;
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
		margin-bottom: 2em;
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
