import imagePresets, { hdPreset, densityPreset } from 'vite-plugin-image-presets';
import { VitePluginRemoteAssets as remoteAssets } from 'vite-plugin-remote-assets';
import { sveltekit } from '@sveltejs/kit/vite';

import pluginCooklang from './plugins/rollup-plugin-cooklang';
import pluginRawFont from './plugins/vite-plugin-raw-fonts';
import pluginB64 from './plugins/vite-plugin-b64';
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

/** @type {import('vite').UserConfig} */
const config = {
    plugins: [
        sveltekit(),
        pluginGlob(),
        pluginB64(),
        pluginCooklang(['.cook']),
        pluginRawFont(['.ttf']),
        process.env.NODE_ENV == 'production'
            ? [
                // remoteAssets({
                //     assetsDir: 'src/lib/records/covers',
                //     awaitDownload: true,
                //     resolveMode: 'relative',
                //     rules: [
                //         {
                //             match: /https:\/\/i\.discogs\.com.*?\.(?:png|jpeg|jpg)/gi
                //         }
                //     ]
                // })
            ]
            : [],
        imagePresets({
            hd: hdPreset({
                widths: [440, 700],
                loading: 'lazy',
                sizes: '(min-width: 700px) 700px, 100vw',
                formats: {
                    avif: { quality: 44 },
                    webp: { quality: 44 },
                    jpg: { quality: 50 }
                },
                inferDimensions: true
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
};

export default config;
