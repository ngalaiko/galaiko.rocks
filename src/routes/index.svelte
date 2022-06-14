<script>
	import GlitchyLinks from '$lib/GlitchyLinks.svelte';
	import Image from '$lib/Image.svelte';
	import nikita from '$lib/assets/people/nikita.jpeg?preset=avatar';
</script>

<svelte:head>
	<title>Nikita Galaiko</title>
</svelte:head>

<article class="content h-card">
	<header class="flex items-center gap-2">
		<h1 class="p-name text-9xl font-bold flex-1">
			<span class="p-given-name">Nikíta</span>
			<span class="p-family-name">Galáiko</span>
		</h1>
		<link class="u-url" href="https://galaiko.rocks" />
		<Image class="u-photo" src={nikita} zoomable={false} />
	</header>

	<GlitchyLinks>
		<p>hi, welcome to my homepage!</p>

		<p>
			if you want to contact me for any reason, the best way is to send an email to
			<a class="u-email" href="mailto:nikita@galaiko.rocks" rel="me">nikita@galaiko.rocks</a>.
			consider using my
			<a sveltekit:reload class="u-key" href="/keys/nikita@galaiko.rocks.asc" rel="me pgpkey authn"
				>pgp key</a
			> if you are into it
		</p>

		<p>this is not a very large website, it only consists of a few pages, for example:</p>
		<ul>
			<li>
				there is a page on this website with a <a sveltekit:prefetch href="/records/"
					>list of vinyl records i own</a
				>. it mirrors my
				<a sveltekit:reload href="https://www.discogs.com/user/ngalaiko">discogs account</a>
			</li>

			<li>
				and here are <a sveltekit:prefetch href="/restaurants_and_cafes/"
					>30 restaurants and cafes</a
				>
				i went to most often last year (updated weekly)
			</li>

			<li>
				<a sveltekit:prefetch href="/cocktails/">a list of cocktails</a> that i respect and occasionaly
				make to consume at home
			</li>
		</ul>

		<p>
			i also have a <a sveltekit:reload href="https://github.com/ngalaiko" rel="me">github</a> account
			if you are interested
		</p>

		<h3>currently</h3>
		<ul>
			<li>
				living in <span class="p-locality">Stockholm</span>,
				<span class="p-country-name">Sweden</span>
			</li>
			<li>working on <a href="https://getsturdy.com">code collaboration</a></li>
			<li><a href="https://codeball.ai">fighting code review</a></li>
		</ul>
	</GlitchyLinks>
</article>
