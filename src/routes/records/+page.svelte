<script lang="ts">
  import Image from '$lib/Image.svelte';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<svelte:head>
  <title>Vinyl Records</title>
  <meta property="og:type" content="website" />
  <meta property="og:title" content="Vinyl Records" />
  <meta property="og:description" content="Vinyl Records i own." />
</svelte:head>

<article>
  <h1>Vinyl Records</h1>
  <p>here are all vinyl records that i have:</p>

  <ul>
    {#each data.records as { artist, info, image }}
      {@const fullName = `${artist.name} - ${info.title}`}
      {@const discogsUrl = `https://www.discogs.com/release/${info.id}`}
      <li>
        <a href={discogsUrl}>
          <figure>
            {#if image}
              <Image src={image} alt={fullName} zoomable={false} />
            {:else}
              <img src={info.coverImage} alt={fullName} />
            {/if}
            <figcaption>{fullName}</figcaption>
          </figure>
        </a>
      </li>
    {/each}
  </ul>
</article>

<style>
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  @media (min-width: 1024px) {
    ul {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }
</style>
