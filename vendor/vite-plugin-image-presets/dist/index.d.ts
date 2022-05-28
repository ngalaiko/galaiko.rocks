import { Plugin } from 'vite';
import * as sharp from 'sharp';
import { Sharp, AvifOptions, GifOptions, HeifOptions, JpegOptions, PngOptions, TiffOptions, WebpOptions, ResizeOptions } from 'sharp';
import { OutputAsset } from 'rollup';

declare function createImageApi(config: Config): {
    readonly config: Config;
    getImageById(id: string): Promise<sharp.Sharp>;
    waitForImages(): Promise<OutputAsset[]>;
    writeImages(outDir: string): Promise<void>;
    purgeCache(assets: OutputAsset[]): Promise<void>;
    resolveImage(filename: string, params: Record<string, any>): Promise<undefined | string | ImageAttrs[]>;
};

declare type ImageApi = ReturnType<typeof createImageApi>;
declare type Image = Sharp;
interface ImageFormatOptions {
    avif: AvifOptions;
    gif: GifOptions;
    heif: HeifOptions;
    jpeg: JpegOptions;
    jpg: JpegOptions;
    png: PngOptions;
    tiff: TiffOptions;
    tif: TiffOptions;
    webp: WebpOptions;
}
declare type ImageFormats = Partial<ImageFormatOptions>;
declare type ImageFormat = keyof ImageFormatOptions;
declare type ImageAttrs = Partial<HTMLImageElement> & {
    class?: string;
};
declare type ImageGeneratorArgs = Record<string, any>;
declare type ImageGenerator = (image: Image, args: ImageGeneratorArgs) => Image | Promise<Image>;
interface ImageSpec {
    /**
     * A condition descriptor that specifies when the image should be used.
     * Can be a width descriptor or a density descriptor.
     * https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset
     */
    condition?: string;
    /**
     * A function to generate the image.
     */
    generate: ImageGenerator;
    /**
     * The parameters for the image generation function.
     */
    args: ImageGeneratorArgs;
}
interface ImageSource {
    type?: string;
    media?: string;
    sizes?: string;
    srcset: ImageSpec[];
}
interface ImagePreset {
    attrs?: ImageAttrs;
    images: ImageSource[];
}
declare type ImagePresets = Record<string, ImagePreset>;
interface Options {
    /**
     * The directory in which to place processed images, relative to Vite's `outDir`.
     * @default 'assets/images'
     */
    assetsDir?: string;
    /**
     * The directory to use for caching images between builds.
     * @default 'node_modules/.images'
     */
    cacheDir?: string;
    /**
     * Definitions of image presets to apply.
     */
    presets?: ImagePresets;
    /**
     * URL parameter that specifies the image preset.
     * @default 'preset'
     */
    urlParam?: string;
    /**
     * Whether to remove cached files that are no longer used.
     * @default true
     */
    purgeCache?: boolean;
    /**
     * Whether to write generated images in the bundle.
     * @default true
     */
    writeToBundle?: boolean;
}
interface Config extends Required<Options> {
    isBuild: boolean;
    base: string;
    root: string;
}

declare function formatFor(image: Image): Promise<ImageFormat>;
declare function mimeTypeFor(format: ImageFormat | 'original'): string | undefined;

declare type FormatOptions = ImageFormats & {
    original?: {};
};
interface WidthPresetOptions extends ImageAttrs {
    density?: number;
    widths: (number | 'original')[];
    formats: FormatOptions;
    resizeOptions?: ResizeOptions;
    withImage?: ImageGenerator;
    media?: string;
}

declare function formatPreset(options: Omit<WidthPresetOptions, 'widths'>): ImagePreset;
declare function hdPreset(options: WidthPresetOptions): {
    attrs: ImageAttrs | undefined;
    images: ImageSource[];
};
declare function widthPreset({ density, widths, formats, resizeOptions, withImage, ...options }: WidthPresetOptions): ImagePreset;
interface DensityPresetOptions extends ImageAttrs {
    density: number[];
    baseHeight?: number;
    baseWidth?: number;
    formats: FormatOptions;
    resizeOptions?: ResizeOptions;
    withImage?: ImageGenerator;
    media?: string;
}
declare function densityPreset({ baseWidth, baseHeight, density, formats, resizeOptions, withImage, ...options }: DensityPresetOptions): ImagePreset;
declare function extractSourceAttrs({ media, sizes, ...attrs }: any): [ImageAttrs, Partial<ImageSource>];

declare function ImagePresetsPlugin(presets?: ImagePresets, options?: Options): Plugin & {
    api: ImageApi;
};

export { Config, Image, ImageApi, ImageAttrs, ImageFormat, ImageFormatOptions, ImageFormats, ImageGenerator, ImageGeneratorArgs, ImagePreset, ImagePresets, ImageSource, ImageSpec, Options, ImagePresetsPlugin as default, densityPreset, extractSourceAttrs, formatFor, formatPreset, hdPreset, mimeTypeFor, widthPreset };
