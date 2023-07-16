declare module '*?b64' {
	export default string;
}

declare module '*?preset=hd' {
	const src: import('vite-plugin-image-presets').ImageAttrs[];
	export default src;
}

declare module '*?preset=avatar' {
	const src: import('vite-plugin-image-presets').ImageAttrs[];
	export default src;
}

declare module '*.cook' {
	const recipe: import('@cooklang/cooklang-ts').ParseResult;
	export default recipe;
}
