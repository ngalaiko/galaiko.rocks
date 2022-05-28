var __defProp = Object.defineProperty;
var __defProps = Object.defineProperties;
var __getOwnPropDescs = Object.getOwnPropertyDescriptors;
var __getOwnPropSymbols = Object.getOwnPropertySymbols;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __propIsEnum = Object.prototype.propertyIsEnumerable;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __spreadValues = (a, b) => {
  for (var prop in b || (b = {}))
    if (__hasOwnProp.call(b, prop))
      __defNormalProp(a, prop, b[prop]);
  if (__getOwnPropSymbols)
    for (var prop of __getOwnPropSymbols(b)) {
      if (__propIsEnum.call(b, prop))
        __defNormalProp(a, prop, b[prop]);
    }
  return a;
};
var __spreadProps = (a, b) => __defProps(a, __getOwnPropDescs(b));
var __restKey = (key) => typeof key === "symbol" ? key : key + "";
var __objRest = (source, exclude) => {
  var target = {};
  for (var prop in source)
    if (__hasOwnProp.call(source, prop) && exclude.indexOf(prop) < 0)
      target[prop] = source[prop];
  if (source != null && __getOwnPropSymbols)
    for (var prop of __getOwnPropSymbols(source)) {
      if (exclude.indexOf(prop) < 0 && __propIsEnum.call(source, prop))
        target[prop] = source[prop];
    }
  return target;
};

// src/index.ts
import { promises as fs3 } from "fs";
import { join as join2 } from "pathe";
import serialize from "@nuxt/devalue";

// src/utils.ts
import { createHash } from "crypto";
import { promises as fs, constants as fsConstants } from "fs";
import sharp from "sharp";
function loadImage(url) {
  return sharp(decodeURIComponent(parseURL(url).pathname));
}
function parseURL(rawURL) {
  return new URL(rawURL.replace(/#/g, "%23"), "file://");
}
function generateImageID(url, args) {
  return createHash("sha256").update(url).update(JSON.stringify(args)).digest("hex").slice(0, 8) + (args.format && args.format !== "original" ? `.${args.format}` : "");
}
function getAssetHash(content) {
  return createHash("sha256").update(content).digest("hex").slice(0, 8);
}
async function exists(path) {
  return await fs.access(path, fsConstants.F_OK).then(() => true, () => false);
}
async function formatFor(image) {
  var _a;
  let format = (_a = image.options) == null ? void 0 : _a.formatOut;
  if (format === "input")
    format = (await image.metadata()).format;
  if (!format) {
    console.error("Could not infer image format for", image);
    throw new Error("Could not infer image format");
  }
  if (format === "heif")
    return "avif";
  return format;
}
function mimeTypeFor(format) {
  if (format === "original")
    return void 0;
  if (format === "jpg")
    format = "jpeg";
  return `image/${format}`;
}
function cleanObject(object) {
  Object.keys(object).forEach((key) => {
    const value = object[key];
    if (value === void 0 || value === null)
      delete object[key];
    else if (isObject(value))
      cleanObject(value);
  });
  return object;
}
function isObject(value) {
  return Object.prototype.toString.call(value) === "[object Object]";
}
function last(arr) {
  return arr[arr.length - 1];
}
function extractSrc(srcset) {
  return srcset ? last(srcset.split(", ")).split(" ")[0] : "";
}

// src/api.ts
import { promises as fs2 } from "fs";
import { basename, join, resolve, extname } from "pathe";
import createDebugger from "debug";
var debug = {
  load: createDebugger("image-presets:load"),
  write: createDebugger("image-presets:write"),
  total: createDebugger("image-presets:total"),
  cache: createDebugger("image-presets:cache")
};
var VIRTUAL_ID = "/@imagepresets/";
function createImageApi(config) {
  const requestedImagesById = {};
  const generatedImages = [];
  const imageHashesByFile = {};
  const imageFilenamesById = {};
  return {
    get config() {
      return config;
    },
    async getImageById(id) {
      return await requestedImagesById[id];
    },
    async waitForImages() {
      debug.total("%i image(s)", generatedImages.length);
      return await Promise.all(generatedImages);
    },
    async writeImages(outDir) {
      debug.total("%i image(s)", generatedImages.length);
      const images = await Promise.all(generatedImages.map(async (imagePromise) => {
        const image = await imagePromise;
        fs2.writeFile(join(outDir, image.fileName), image.source);
        return image;
      }));
      this.purgeCache(images);
    },
    async purgeCache(assets) {
      if (!config.purgeCache)
        return;
      const usedFiles = new Set(assets.map((asset) => asset.name));
      const cachedFiles = await fs2.readdir(config.cacheDir);
      const unusedFiles = cachedFiles.filter((file) => !usedFiles.has(file));
      debug.cache("%i unused files", unusedFiles.length);
      unusedFiles.forEach((file) => {
        fs2.rm(resolve(config.cacheDir, file), { force: true });
      });
    },
    async resolveImage(filename, params) {
      var _a;
      const _b = params, { [_a = config.urlParam]: presetName, src, srcset } = _b, otherParams = __objRest(_b, [__restKey(_a), "src", "srcset"]);
      const preset = config.presets[presetName];
      debug.load("%O %s", params, filename);
      if (!preset)
        throw new Error(`vite-image-presets: Unknown image preset '${presetName}'`);
      const imagesAttrs = await Promise.all(preset.images.map(async (_c) => {
        var _d = _c, { srcset: srcset2 } = _d, source = __objRest(_d, ["srcset"]);
        return __spreadProps(__spreadValues({}, source), {
          srcset: (await Promise.all(srcset2.map(async ({ condition, args, generate }) => [
            encodeURI(await getImageSrc(filename, __spreadValues(__spreadValues({}, args), otherParams), generate)),
            condition
          ].filter((x) => x).join(" ")))).join(", ")
        });
      }));
      const lastImage = last(imagesAttrs);
      const lastSrc = extractSrc(lastImage.srcset);
      if (src !== void 0)
        return lastSrc;
      if (srcset !== void 0) {
        const attrs = imagesAttrs[srcset === "" ? imagesAttrs.length - 1 : Number(srcset)];
        if (!attrs)
          throw new Error(`The '${presetName}' image preset did not provide any source matching the provided index: ${srcset}.
URL: ${filename}?${new URLSearchParams(params)}`);
        return attrs.srcset;
      }
      Object.assign(lastImage, preset.attrs);
      lastImage.src || (lastImage.src = lastSrc);
      return imagesAttrs;
    }
  };
  async function getImageHash(filename) {
    return await (imageHashesByFile[filename] || (imageHashesByFile[filename] = loadImage(filename).toBuffer().then(getAssetHash)));
  }
  async function queueImageAndGetFilename(id, sourceFilename, image) {
    const base = basename(sourceFilename, extname(sourceFilename));
    const hash = getAssetHash(id + await getImageHash(sourceFilename));
    const format = await formatFor(image);
    const filename = `${base}.${hash}.${format}`;
    generatedImages.push(writeImageFile(filename, image));
    return join(config.assetsDir, filename);
  }
  async function writeImageFile(filename, image) {
    const { cacheDir, assetsDir } = config;
    const cachedFilename = join(cacheDir, filename);
    if (!await exists(cachedFilename)) {
      debug.write("%s", cachedFilename);
      await image.toFile(cachedFilename);
    }
    return {
      fileName: join(assetsDir, filename),
      name: filename,
      source: await fs2.readFile(cachedFilename),
      isAsset: true,
      type: "asset"
    };
  }
  async function getImageSrc(filename, args, generate) {
    filename = resolve(config.root, filename);
    const id = generateImageID(filename, args);
    requestedImagesById[id] || (requestedImagesById[id] = generate(loadImage(filename), args));
    if (config.isBuild) {
      const image = await requestedImagesById[id];
      imageFilenamesById[id] || (imageFilenamesById[id] = queueImageAndGetFilename(id, filename, image));
      return config.base + await imageFilenamesById[id];
    }
    return VIRTUAL_ID + id;
  }
}

// src/presets.ts
function formatPreset(options) {
  return widthPreset(__spreadValues({ widths: ["original"] }, options));
}
function hdPreset(options) {
  const highDensity = widthPreset(__spreadValues({ density: 2, media: "(-webkit-min-device-pixel-ratio: 1.5)" }, options));
  const desktopWidth = Math.max(...options.widths) || "original";
  const desktop = widthPreset(__spreadProps(__spreadValues({}, options), { widths: [desktopWidth] }));
  return { attrs: desktop.attrs, images: highDensity.images.concat(desktop.images) };
}
function widthPreset(_a) {
  var _b = _a, { density, widths, formats, resizeOptions, withImage } = _b, options = __objRest(_b, ["density", "widths", "formats", "resizeOptions", "withImage"]);
  const [attrs, sourceAttrs] = extractSourceAttrs(options);
  return {
    attrs,
    images: Object.entries(formats).map(([format, formatOptions]) => __spreadProps(__spreadValues({}, sourceAttrs), {
      type: mimeTypeFor(format),
      srcset: widths.map((width) => cleanObject({
        condition: width === "original" ? void 0 : `${width}w`,
        args: { preset: "width", format, width, density, formatOptions, resizeOptions },
        generate: async (image, args) => {
          if (format !== "original")
            image = image.toFormat(format, formatOptions);
          if (width !== "original") {
            const hdWidth = density ? width * density : width;
            image = image.resize(__spreadValues({ width: hdWidth, withoutEnlargement: true }, resizeOptions));
          }
          return await (withImage == null ? void 0 : withImage(image, args)) || image;
        }
      }))
    }))
  };
}
function multiply(quantity, n) {
  return n ? quantity * n : void 0;
}
function densityPreset(_a) {
  var _b = _a, { baseWidth, baseHeight, density, formats, resizeOptions, withImage } = _b, options = __objRest(_b, ["baseWidth", "baseHeight", "density", "formats", "resizeOptions", "withImage"]);
  const [attrs, sourceAttrs] = extractSourceAttrs(options);
  return {
    attrs,
    images: Object.entries(formats).map(([format, formatOptions]) => __spreadProps(__spreadValues({}, sourceAttrs), {
      type: mimeTypeFor(format),
      srcset: density.map((density2) => cleanObject({
        condition: `${density2}x`,
        args: { preset: "density", format, density: density2, baseWidth, baseHeight, formatOptions, resizeOptions },
        generate: async (image, args) => {
          if (format !== "original")
            image = image.toFormat(format, formatOptions);
          if (baseWidth || baseHeight) {
            image = image.resize(__spreadValues({
              width: multiply(density2, baseWidth),
              height: multiply(density2, baseHeight),
              withoutEnlargement: true
            }, resizeOptions));
          }
          return await (withImage == null ? void 0 : withImage(image, args)) || image;
        }
      }))
    }))
  };
}
function extractSourceAttrs(_a) {
  var _b = _a, { media, sizes } = _b, attrs = __objRest(_b, ["media", "sizes"]);
  return [cleanObject(__spreadValues({ loading: "lazy" }, attrs)), cleanObject({ media, sizes })];
}

// src/index.ts
function ImagePresetsPlugin(presets, options) {
  let api;
  let config;
  return {
    name: "image-presets",
    enforce: "pre",
    get api() {
      return api;
    },
    async configResolved({ base, command, root, build: { assetsDir } }) {
      if (api)
        return;
      config = __spreadValues({
        presets,
        urlParam: "preset",
        base,
        root,
        assetsDir,
        cacheDir: join2(root, "node_modules", ".images"),
        purgeCache: true,
        writeToBundle: true,
        isBuild: command === "build"
      }, options);
      api = createImageApi(config);
      if (config.isBuild)
        await fs3.mkdir(config.cacheDir, { recursive: true });
    },
    async load(id) {
      if (!id.includes(config.urlParam))
        return;
      const { path, query } = parseId(id);
      if (!query.preset)
        return;
      const images = await api.resolveImage(path, query);
      return `export default ${serialize(images)}`;
    },
    configureServer(server) {
      server.middlewares.use(async (req, res, next) => {
        var _a;
        if ((_a = req.url) == null ? void 0 : _a.startsWith(VIRTUAL_ID)) {
          const [, id] = req.url.split(VIRTUAL_ID);
          const image = await api.getImageById(id);
          if (!image) {
            console.error(`vite-image-presets cannot find image with id "${id}" this is likely an internal error`);
            res.statusCode = 404;
            return res.end();
          }
          res.setHeader("Content-Type", `image/${await formatFor(image)}`);
          res.setHeader("Cache-Control", "max-age=360000");
          return image.clone().on("error", (err) => console.error(err)).pipe(res);
        }
        next();
      });
    },
    async generateBundle(_, output) {
      if (config.writeToBundle) {
        const images = await api.waitForImages();
        images.forEach((asset) => {
          output[asset.fileName] = asset;
        });
        api.purgeCache(images);
      }
    }
  };
}
function parseId(id) {
  const index = id.indexOf("?");
  if (index < 0)
    return { path: id, query: {} };
  const query = Object.fromEntries(new URLSearchParams(id.slice(index)));
  return { path: id.slice(0, index), query };
}
export {
  ImagePresetsPlugin as default,
  densityPreset,
  extractSourceAttrs,
  formatFor,
  formatPreset,
  hdPreset,
  mimeTypeFor,
  widthPreset
};
