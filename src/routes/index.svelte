<script>
	import Image from 'svelte-image';
	import GlitchyLinks from '$lib/GlitchyLinks.svelte';
</script>

<svelte:head>
    <title>Nikita Galaiko</title>
</svelte:head>

<header class="flex items-center gap-2">
	<h1 class="p-name text-4xl font-bold flex-1">Nikita Galaiko</h1>
	<div class="w-[50px] h-[50px]">
		<Image
			src="img/people/nikita.jpeg"
			class="rounded-full"
			alt="Nikita Galaiko's avatar"
            lazy={false}
		/>
	</div>
</header>

<div class="e-content">
	<GlitchyLinks>
		<p>hi, welcome to my homepage!</p>

		<p>
			if you want to contact me for any reason, the best way is to send an email to
			<a href="mailto:nikita@galaiko.rocks" rel="me">nikita@galaiko.rocks</a>. consider using my
			<a href="/keys/nikita@galaiko.rocks.asc" rel="pgpkey authn">pgp key</a> if you are into it
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
			<li>living in Stockholm</li>
			<li>working on <a href="https://getsturdy.com">code collaboration</a></li>
		</ul>
	</GlitchyLinks>
</div>
