<script lang="ts">
	import { Article } from '$lib/components';
	import Image from '$lib/Image.svelte';
	import me from '$lib/assets/people/nikita.jpeg?preset=avatar';
</script>

<Article>
	<div class="h-entry">
		<div class="flex gap-2">
			<a class="p-author h-card flex gap-2" href="http://galaiko.rocks">
				<Image src={me} />
				Nikita Galaiko</a
			>
			<span>
				in reply to <a
					class=" u-in-reply-to underline"
					href="https://galaiko.rocks/posts/blog/hello-indieweb/">Hello, IndieWeb!</a
				>:
			</span>
		</div>
		<span class="p-content content pl-2">It works!</span>
	</div>
</Article>
