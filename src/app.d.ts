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
