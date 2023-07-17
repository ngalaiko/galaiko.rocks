<script lang="ts">
  import type { PageData } from './$types';
  import { addYears, formatRelative } from 'date-fns';
  import type { Map, Marker } from 'leaflet';

  export let data: PageData;

  export let markers: Marker[] = [];

  let m: Map;
  let mapElement: HTMLElement;
  const select = (payee: string) => {
    const index = data.places.findIndex((p) => p.payee === payee);
    if (index === -1) return;
    mapElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
    markers[index].openPopup();
    m.flyTo(data.places[index].location as [number, number], 16);
  };

  const map = (map: HTMLElement) => {
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

      markers = data.places.map((place) =>
        L.marker(place.location as [number, number], {
          icon: L.divIcon({
            html: document.createElement('div'),
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
          .addTo(m)
      );
    });
    return { destroy: () => m.remove() };
  };
</script>

<svelte:head>
  <title>Restaurants and Cafes</title>
  <meta property="og:title" content="Restaurants and Cafes" />
  <meta
    property="og:description"
    content="Restaurants and cafes I went to the most since last year."
  />
  <meta property="og:type" content="website" />
</svelte:head>

<article>
  <h1>Restaurants and Cafes</h1>
  <p>
    food establishments i went to the most since {formatRelative(
      addYears(new Date(), -1),
      new Date()
    )}:
  </p>

  <div use:map bind:this={mapElement} />

  <table>
    <thead>
      <tr>
        <th style:text-align="left" scope="col"><h2>Place</h2></th>
        <th style:text-align="center" scope="col"><h2>Times</h2></th>
        <th style:text-align="right" scope="col"><h2>Spent</h2></th>
      </tr>
      {#each data.places as { payee, count, amount, currency }}
        <tr>
          <td style:text-align="left">
            <button on:click={() => select(payee)}>
              {payee}
            </button>
          </td>
          <td style:text-align="center">{count}</td>
          <td style:text-align="right">{amount.toLocaleString()} {currency}</td>
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
    table-layout: fixed;
    white-space: nowrap;
    width: 100%;
    text-align: left;
  }

  th,
  td {
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0 0.5rem;
  }

  button {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    cursor: pointer;
    color: inherit;
    outline: inherit;
  }

  button:hover {
    text-decoration: underline;
  }
</style>
