<script>
	import A from '$lib/GlitchyLink.svelte';
	import Image from '$lib/Image.svelte';
	import nikita from '$lib/assets/people/nikita.jpeg?preset=avatar';
</script>

<svelte:head>
	<title>Nikita Galaiko</title>
	<meta property="og:title" content="Nikita Galaiko" />
	<meta property="og:type" content="profile" />
	<meta property="profile:first_name" content="Nikita" />
	<meta property="profile:last_name" content="Galaiko" />
	<meta property="profile:username" content="ngalaiko" />
	<meta property="profile:gender" content="male" />
</svelte:head>

<article class="h-card">
	<header>
		<h1 class="p-name">
			<span class="p-given-name">Nikita</span>
			<span class="p-family-name">Galaiko</span>
		</h1>
		<link class="u-url" href="https://galaiko.rocks" />
		<Image class="u-photo" src={nikita} zoomable={false} />
	</header>

	<p>hi, welcome to my homepage!</p>

	<p>
		if you want to contact me for any reason, the best way is to send an email to
		<A class="u-email" href="mailto:nikita@galaiko.rocks" rel="me">nikita@galaiko.rocks</A>.
		consider using my
		<A
			data-sveltekit-reload
			class="u-key"
			href="/keys/nikita@galaiko.rocks.asc"
			rel="me pgpkey authn">pgp key</A
		> if you are into it
	</p>

	<p>this is not a very large website, it only consists of a few pages, for example:</p>
	<ul>
		<li>
			there is a page on this website with a <A data-sveltekit-prefetch href="/records/"
				>list of vinyl records i own</A
			>. it mirrors my
			<A data-sveltekit-reload href="https://www.discogs.com/user/ngalaiko">discogs account</A>
		</li>

		<li>
			and here are <A data-sveltekit-prefetch href="/restaurants_and_cafes/"
				>restaurants and cafes</A
			>
			i went to at least twice since last year (updated weekly)
		</li>

		<li>
			<A data-sveltekit-prefetch href="/cocktails/">a list of cocktails</A> that i respect and occasionaly
			make to consume at home
		</li>

		<li>
			<A data-sveltekit-prefetch href="/uses/">list of things i use</A>
		</li>
	</ul>

	<p>
		i also have a <A data-sveltekit-reload href="https://github.com/ngalaiko" rel="me">github</A> account
		if you are interested
	</p>

	<h2>currently</h2>
	<ul>
		<li>
			living in <span class="p-locality">Göteborg</span>,
			<span class="p-country-name">Sweden</span>
		</li>
		<li>working on the <A href="https://gitbutler.com">future of source code management</A></li>
	</ul>
</article>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
