<script lang="ts">
	import type { PageData } from './$types';
	import { addYears, formatRelative } from 'date-fns';
	import type { Map } from 'leaflet';

	export let data: PageData;

	const map = (map: HTMLElement) => {
		let m: Map;
		import('leaflet/dist/leaflet.css').then(async () => {
			const L = await import('leaflet');

			// expand the div to make it visible when js is on
			map.classList.add('h-[400px]');

			// init map with stockholm in the middle
			m = L.map(map, {
				center: [59.32799, 18.05467],
				zoom: 12
			});

			// use carto maps as a base layer
			L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
				attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
		    &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
				subdomains: 'abcd',
				maxZoom: 16
			}).addTo(m);

			const max = data.places.map((p) => p.count).sort((a, b) => b - a)[0];

			data.places.forEach((place) => {
				const div = document.createElement('div');
				L.marker(place.location as [number, number], {
					icon: L.divIcon({
						html: div,
						className: `rounded-full h-4 w-4 bg-orange`
					}),
					title: place.payee,
					alt: place.payee,
					riseOnHover: true
				})
					.setOpacity(0.2 + (place.count / max) * 0.8)
					.bindPopup(
						[
							`<b>${place.payee}</b>`,
							`visits: ${place.count}`,
							`spent: ${place.amount} ${place.currency}`
						].join('<br/>')
					)
					.addTo(m);
			});
		});
		return { destroy: () => m.remove() };
	};
</script>

<svelte:head>
	<title>Restaurants and Cafes</title>
</svelte:head>

<article class="flex flex-col gap-2">
	<h1 class="text-2xl text-bold">Restaurants and Cafes</h1>
	<p>
		food establishments i went to the most since {formatRelative(
			addYears(new Date(), -1),
			new Date()
		)}:
	</p>

	<div use:map />

	<table class="table-auto w-full">
		<thead>
			<tr class="text-gray text-md font-semibold text-left">
				<th scope="col" class="pt-3 px-2">Place</th>
				<th scope="col" class="pt-3 px-2">Times</th>
				<th scope="col" class="pt-3 px-2">Spent</th>
			</tr>
			{#each data.places as { payee, count, amount, currency }}
				<tr>
					<td class="md:whitespace-nowrap px-2 md:px-3 py-4">{payee}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{count}</td>
					<td class="whitespace-nowrap px-2 md:px-3 py-4">{amount.toLocaleString()} {currency}</td>
				</tr>
			{/each}
		</thead>
	</table>
</article>
