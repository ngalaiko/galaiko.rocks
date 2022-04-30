/// <reference types="@sveltejs/kit" />

declare namespace App {
	interface Locals {}

	interface Platform {
		env: {
			WEB_MENTIONS: KVNamespace;
		};
	}

	interface Session {}

	interface Stuff {}
}

declare module '*?preset=hd' {
	const src: import('vite-plugin-image-presets').ImageAttrs[];
	export default src;
}

declare module '*?preset=avatar' {
	const src: import('vite-plugin-image-presets').ImageAttrs[];
	export default src;
}
