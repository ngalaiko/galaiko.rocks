<script lang="ts">
  import { format } from 'date-fns';
  import type { PageData } from './$types';

  export let data: PageData;
  console.log(data);
</script>

<svelte:head>
  <title>movies</title>
  <meta property="og:type" content="website" />
  <meta property="og:title" content="movies" />
  <meta property="og:description" content="movies i watch" />
</svelte:head>

<article>
  <ul class="h-feed">
    {#each data.movies as movie, i}
      {@const date = movie.watchedDate}
      {@const prevDate = data.movies[i - 1]?.watchedDate}
      {@const day = date.getDate()}
      {#if i == 0 || date.getFullYear() !== prevDate.getFullYear()}
        <h2>
          <time datetime={date.toISOString()}>
            <span>{format(date, 'yyyy')}</span>
            <span>{format(date, 'MMMM')}</span>
          </time>
        </h2>
      {:else if date.getMonth() != prevDate.getMonth()}
        <h2>
          <time datetime={date.toISOString()}>
            <span />
            <span>{format(date, 'MMMM')}<span /></span></time
          >
        </h2>
      {/if}
      <li class="h-entry">
        <a class="u-url" href={movie.href}>
          <span class="p-name">
            {movie.title}
          </span>
          <hr />
          <time class="dt-published" datetime={date.toISOString()}>
            {#if day == 1 || day == 21 || day == 31}
              {day}st
            {:else if day == 2 || day == 22}
              {day}nd
            {:else if day == 3 || day == 23}
              {day}rd
            {:else}
              {day}th
            {/if}
          </time>
        </a>
      </li>
    {/each}
  </ul>
</article>

<style>
  a {
    color: inherit;
    display: flex;
    padding: 0.5rem 0;
  }

  a:visited {
    color: inherit;
  }

  time {
    display: flex;
    justify-content: space-between;
  }

  ul {
    padding: 0;
    list-style-type: none;
  }

  hr {
    border: none;
    flex: auto;
    align-self: flex-end;
    margin: 0 0.3rem 0.3rem;
    border-top: 1px dotted;
  }

  h2 {
    text-align: justify;
  }
</style>
