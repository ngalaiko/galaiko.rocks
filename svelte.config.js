import adapter from '@sveltejs/adapter-auto';
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
        preprocess({
            postcss: true,
            typescript: true,
            replace: [['import.meta.env.GITHUB_TOKEN', JSON.stringify(process.env.GITHUB_TOKEN)]]
        })
    ],
    kit: {
        adapter: adapter(),
        prerender: {
            enabled: true,
            crawl: true,
            entries: [
                '/posts/' //  it's not linked from anywhere
            ]
        },
        trailingSlash: 'always'
    }
};

export default config;
