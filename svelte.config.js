import adapter from '@sveltejs/adapter-cloudflare';
import preprocess from 'svelte-preprocess';

import { mdsvex } from 'mdsvex';
import slug from 'rehype-slug';
import autoLinkHeadings from 'rehype-autolink-headings';

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
				slug, // adds slug to headers
				[autoLinkHeadings, { behavior: 'wrap' }] //  adds a <a> around slugged headers
			]
		}),
		preprocess({ postcss: true, typescript: true })
	],
	kit: {
		adapter: adapter(),
		prerender: {
			default: true,
			enabled: true,
			crawl: true,
			entries: [
				'/posts/', //  it's not linked from anywhere
			]
		},
		trailingSlash: 'always'
	}
};

export default config;
