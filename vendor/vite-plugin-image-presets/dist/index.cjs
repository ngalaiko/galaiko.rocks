var __create = Object.create;
var __defProp = Object.defineProperty;
var __defProps = Object.defineProperties;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropDescs = Object.getOwnPropertyDescriptors;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getOwnPropSymbols = Object.getOwnPropertySymbols;
var __getProtoOf = Object.getPrototypeOf;
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
var __markAsModule = (target) => __defProp(target, "__esModule", { value: true });
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
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __reExport = (target, module2, copyDefault, desc) => {
  if (module2 && typeof module2 === "object" || typeof module2 === "function") {
    for (let key of __getOwnPropNames(module2))
      if (!__hasOwnProp.call(target, key) && (copyDefault || key !== "default"))
        __defProp(target, key, { get: () => module2[key], enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable });
  }
  return target;
};
var __toESM = (module2, isNodeMode) => {
  return __reExport(__markAsModule(__defProp(module2 != null ? __create(__getProtoOf(module2)) : {}, "default", !isNodeMode && module2 && module2.__esModule ? { get: () => module2.default, enumerable: true } : { value: module2, enumerable: true })), module2);
};
var __toCommonJS = /* @__PURE__ */ ((cache) => {
  return (module2, temp) => {
    return cache && cache.get(module2) || (temp = __reExport(__markAsModule({}), module2, 1), cache && cache.set(module2, temp), temp);
  };
})(typeof WeakMap !== "undefined" ? /* @__PURE__ */ new WeakMap() : 0);

// src/index.ts
var src_exports = {};
__export(src_exports, {
  default: () => ImagePresetsPlugin,
  densityPreset: () => densityPreset,
  extractSourceAttrs: () => extractSourceAttrs,
  formatFor: () => formatFor,
  formatPreset: () => formatPreset,
  hdPreset: () => hdPreset,
  mimeTypeFor: () => mimeTypeFor,
  widthPreset: () => widthPreset
});
var import_fs3 = require("fs");
var import_pathe2 = require("pathe");
var import_devalue = __toESM(require("@nuxt/devalue"), 1);

// src/utils.ts
var import_crypto = require("crypto");
var import_fs = require("fs");
var import_sharp = __toESM(require("sharp"), 1);
function loadImage(url) {
  return (0, import_sharp.default)(decodeURIComponent(parseURL(url).pathname));
}
function parseURL(rawURL) {
  return new URL(rawURL.replace(/#/g, "%23"), "file://");
}
function generateImageID(url, args) {
  return (0, import_crypto.createHash)("sha256").update(url).update(JSON.stringify(args)).digest("hex").slice(0, 8) + (args.format && args.format !== "original" ? `.${args.format}` : "");
}
function getAssetHash(content) {
  return (0, import_crypto.createHash)("sha256").update(content).digest("hex").slice(0, 8);
}
async function exists(path) {
  return await import_fs.promises.access(path, import_fs.constants.F_OK).then(() => true, () => false);
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
var import_fs2 = require("fs");
var import_pathe = require("pathe");
var import_debug = __toESM(require("debug"), 1);
var debug = {
  load: (0, import_debug.default)("image-presets:load"),
  write: (0, import_debug.default)("image-presets:write"),
  total: (0, import_debug.default)("image-presets:total"),
  cache: (0, import_debug.default)("image-presets:cache")
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
        import_fs2.promises.writeFile((0, import_pathe.join)(outDir, image.fileName), image.source);
        return image;
      }));
      this.purgeCache(images);
    },
    async purgeCache(assets) {
      if (!config.purgeCache)
        return;
      const usedFiles = new Set(assets.map((asset) => asset.name));
      const cachedFiles = await import_fs2.promises.readdir(config.cacheDir);
      const unusedFiles = cachedFiles.filter((file) => !usedFiles.has(file));
      debug.cache("%i unused files", unusedFiles.length);
      unusedFiles.forEach((file) => {
        import_fs2.promises.rm((0, import_pathe.resolve)(config.cacheDir, file), { force: true });
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
    const base = (0, import_pathe.basename)(sourceFilename, (0, import_pathe.extname)(sourceFilename));
    const hash = getAssetHash(id + await getImageHash(sourceFilename));
    const format = await formatFor(image);
    const filename = `${base}.${hash}.${format}`;
    generatedImages.push(writeImageFile(filename, image));
    return (0, import_pathe.join)(config.assetsDir, filename);
  }
  async function writeImageFile(filename, image) {
    const { cacheDir, assetsDir } = config;
    const cachedFilename = (0, import_pathe.join)(cacheDir, filename);
    if (!await exists(cachedFilename)) {
      debug.write("%s", cachedFilename);
      await image.toFile(cachedFilename);
    }
    return {
      fileName: (0, import_pathe.join)(assetsDir, filename),
      name: filename,
      source: await import_fs2.promises.readFile(cachedFilename),
      isAsset: true,
      type: "asset"
    };
  }
  async function getImageSrc(filename, args, generate) {
    filename = (0, import_pathe.resolve)(config.root, filename);
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
        cacheDir: (0, import_pathe2.join)(root, "node_modules", ".images"),
        purgeCache: true,
        writeToBundle: true,
        isBuild: command === "build"
      }, options);
      api = createImageApi(config);
      if (config.isBuild)
        await import_fs3.promises.mkdir(config.cacheDir, { recursive: true });
    },
    async load(id) {
      if (!id.includes(config.urlParam))
        return;
      const { path, query } = parseId(id);
      if (!query.preset)
        return;
      const images = await api.resolveImage(path, query);
      return `export default ${(0, import_devalue.default)(images)}`;
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
module.exports = __toCommonJS(src_exports);
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  densityPreset,
  extractSourceAttrs,
  formatFor,
  formatPreset,
  hdPreset,
  mimeTypeFor,
  widthPreset
});
