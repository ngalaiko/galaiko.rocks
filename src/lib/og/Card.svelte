<script lang="ts">
	import me from '$lib/assets/people/nikita.jpeg?b64';
	export let message: string | undefined = undefined;
</script>

<main>
	<div>
		<h1>Nikita Galaiko</h1>
		<img src="data:image/jpeg;base64,{me}" width="80" height="80" alt="Nikita Galaiko" />
	</div>
	{#if message}
		<h2>
			{message}
		</h2>
	{/if}
</main>

<style>
	div {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	h1 {
		font-size: 50px;
		color: #d5c4a1;
	}

	h2 {
		font-size: 70px;
		color: #fbf1c7;
	}

	img {
		border-radius: 9999px;
	}

	main {
		padding: 100px;

		display: flex;
		flex-direction: column;
		justify-content: center;

		font-family: 'Iowan Old Style';

		background-color: #32302f;

		height: 100%;
		width: 100%;
	}
</style>
