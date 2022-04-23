<script lang="ts">
	import type { Reply } from '$lib/webmentions';
	import ReplyComponent from './Reply.svelte';

	export let replies: Reply[];
</script>

<div>
	<h2 class="text-xl">{replies.length} replies</h2>
	<ul class="flex flex-col gap-2">
		{#each replies as reply}
			<li class="py-2">
				<ReplyComponent {reply} />
			</li>
		{:else}
			<li class="py-2">
				<p>
					Send one using <a class="text-blue" href="https://indieweb.org/Webmention">Webmentions</a>!
				</p>
			</li>
		{/each}
	</ul>
</div>
