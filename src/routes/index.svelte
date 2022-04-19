<script>
	import Image from 'svelte-image';
	import GlitchyLinks from '$lib/GlitchyLinks.svelte';
</script>

<svelte:head>
	<title>Nikita Galaiko</title>
</svelte:head>

<div class="content h-card">
	<header class="flex items-center gap-2">
		<h1 class="p-name text-4xl font-bold flex-1">
			<span class="p-given-name">Nikita</span>
			<span class="p-family-name">Galaiko</span>
		</h1>
		<link class="u-url" href="https://galaiko.rocks" />
		<div class="w-[50px] h-[50px]">
			<Image
				class="u-photo rounded-full"
				src="img/people/nikita.jpeg"
				alt="Nikita Galaiko's avatar"
				lazy={false}
			/>
		</div>
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

		<p>
			i have a <a href="https://github.com/ngalaiko" rel="me">github</a> account if you want to checkout
			stuff i occasionaly do for fun (programming wise)
		</p>

		<p>
			there is a page on this website with a <a href="/records/">list of vinyl records i own</a>
		</p>

		<p>
			and here is a <a href="/restaurants_and_cafes/">top 30 restaurants and cafes</a> i visited since
			last year (updated weekly!)
		</p>

		<h3>currently</h3>
		<ul>
			<li>
				living in <span class="p-locality">Stockholm</span>,
				<span class="p-country-name">Sweden</span>
			</li>
			<li>working on <a href="https://getsturdy.com">code collaboration</a></li>
		</ul>
	</GlitchyLinks>
</div>
