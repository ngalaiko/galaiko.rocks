import adapter from '@sveltejs/adapter-static';
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
				'/posts/' //  it's not linked from anywhere
			]
		},
		csp: {
			directives: {
				'default-src': ['self'],
				'script-src': ['self', 'static.cloudflareinsights.com'],
				'connect-src': ['self', 'vitals.vercel-insights.com'],
				'style-src': ['self', 'unsafe-inline']
			}
		},
		trailingSlash: 'always'
	}
};

export default config;
