import adapter from '@sveltejs/adapter-cloudflare';
import preprocess from 'svelte-preprocess';

import { mdsvex } from 'mdsvex';
import slug from 'rehype-slug';
import autoLinkHeadings from 'rehype-autolink-headings';
import imagePresets, { hdPreset, densityPreset } from 'vite-plugin-image-presets';

import pluginCooklang from './plugins/rollup-plugin-cooklang.js';
import pluginGlob from 'vite-plugin-glob';

const rectFor = (width, height = width) =>
	Buffer.from(
		`<svg><rect x="0" y="0" width="${width}" height="${height}" rx="${width}" ry="${height}"/></svg>`
	);

const withRoundBorders = (image) => {
	const { width } = image.options;
	return image
		.resize({ width, height: width, fit: 'cover' })
		.composite([{ input: rectFor(width), blend: 'dest-in' }]);
};

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
				'/replies/' //  it's not linked from anywhere
			]
		},
		trailingSlash: 'always',
		vite: {
			plugins: [
				pluginGlob(),
				pluginCooklang(),
				imagePresets({
					hd: hdPreset({
						widths: [440, 700],
						loading: 'lazy',
						sizes: '(min-width: 700px) 700px, 100vw',
						formats: {
							avif: { quality: 44 },
							webp: { quality: 44 },
							jpg: { quality: 50 }
						}
					}),
					avatar: densityPreset({
						height: 48, // avoid layout shift
						baseWidth: 48,
						density: [1, 1.5, 2],
						resizeOptions: {
							fit: 'cover'
						},
						withImage: withRoundBorders,
						formats: {
							webp: { quality: 40 },
							png: { quality: 40 }
						}
					})
				})
			]
		}
	}
};

export default config;
