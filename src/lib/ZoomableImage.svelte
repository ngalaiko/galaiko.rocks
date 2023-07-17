<script lang="ts">
  import type { ImageAttrs } from 'vite-plugin-image-presets';
  import Lightense from 'lightense-images';
  import { onMount } from 'svelte';

  export let attrs: ImageAttrs;
  export let alt = '';

  let img: HTMLImageElement;
  onMount(() => {
    Lightense(img, {
      time: 100,
      keyboard: true,
      cubicBezier: 'cubic-bezier(.2, 0, .1, 1)',
      background: 'var(--background)',
      zIndex: 1e6
    });
  });
</script>

<img {...attrs} {alt} bind:this={img} />

<style>
  img {
    cursor: zoom-in;
  }
</style>
