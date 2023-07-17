import adapter from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';

import { mdsvex } from 'mdsvex';
import slug from 'rehype-slug';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: ['.svelte', '.md'],
  preprocess: [
    mdsvex({
      extensions: ['.md'],
      smartypants: {
        dashes: 'oldschool'
      },
      rehypePlugins: [
        slug // adds slug to headers
      ]
    }),
    preprocess({
      typescript: true,
      replace: [['import.meta.env.GITHUB_TOKEN', JSON.stringify(process.env.GITHUB_TOKEN)]]
    })
  ],
  kit: {
    adapter: adapter(),
    prerender: {
      crawl: true,
      entries: [
        '/posts/' //  it's not linked from anywhere
      ],
      handleMissingId: ({ path, message }) => {
        if (!path.startsWith('/cocktails')) {
          throw new Error(message);
        }
      }
    }
  }
};

export default config;
