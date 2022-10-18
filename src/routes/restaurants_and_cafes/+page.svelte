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
			map.style.height = '400px';

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
						className: 'icon'
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

<article>
	<h1>Restaurants and Cafes</h1>
	<p>
		food establishments i went to the most since {formatRelative(
			addYears(new Date(), -1),
			new Date()
		)}:
	</p>

	<div use:map />

	<table>
		<thead>
			<tr>
				<th scope="col"><h2>Place</h2></th>
				<th scope="col"><h2>Times</h2></th>
				<th scope="col"><h2>Spent</h2></th>
			</tr>
			{#each data.places as { payee, count, amount, currency }}
				<tr>
					<td>{payee}</td>
					<td>{count}</td>
					<td>{amount.toLocaleString()} {currency}</td>
				</tr>
			{/each}
		</thead>
	</table>
</article>

<style>
	:global(.icon) {
		background-color: var(--orange);
		border-radius: 9999px;
	}

	table {
		table-layout: auto;
		width: 100%;
		text-align: left;
	}

	td {
		padding-top: 1rem;
	}
</style>
