<script lang="ts">
	import { Article } from '$lib/components';
	import me from '$lib/assets/people/nikita.jpeg?preset=avatar';
	import { Reply as ReplyComponent } from '$lib/components/molecules';
	import type { Reply, Author } from '$lib/webmentions';
	import { page } from '$app/stores';

	const nikita: Author = {
		picture: new URL(me.slice(-1)[0].src, $page.url.origin).href,
		name: 'Nikita Galaiko',
		url: 'https://galaiko.rocks'
	};

	const href = 'https://galaiko.rocks/replies/';

	const range = (start: number, end: number) => {
		const result = [];
		for (let i = start; i <= end; i++) {
			result.push(i);
		}
		return result;
	};

	const replies: Reply[] = [
		{
			author: nikita,
			content: 'It works!',
			timestamp: new Date('2022-04-30').getTime(),
			source: href,
			target: 'https://galaiko.rocks/posts/blog/hello-indieweb/'
		},
		...range(1, 23).map((i) => ({
			author: nikita,
			content: `Hello! This is a test reply #${i}`,
			timestamp: new Date('2022-05-01').getTime(),
			source: href,
			target: `https://webmention.rocks/test/${i}`
		}))
	].sort((a, b) => a.timestamp - b.timestamp);
</script>

<Article>
	{#each replies as reply}
		<ReplyComponent {reply} detailed />
	{/each}
</Article>
