<script lang="ts">
  import { page } from '$app/stores';
  const title = $page.status === 404 ? 'Page not found.' : 'Something went wrong.';
  console.log($page.error);
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<article>
  <h1 class="w-full h-[50vh] flex items-center justify-center">{title}</h1>
</article>

<style>
  h1 {
    text-align: center;
  }
</style>
