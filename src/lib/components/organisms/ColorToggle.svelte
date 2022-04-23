<script lang="ts">
	import { onMount } from 'svelte';

	type Theme = 'theme-light' | 'theme-dark';

	const getCurrentThemeFromCookie = (): Theme | null => {
		const cookie = document.cookie;
		const theme = cookie.split(';').find((c) => c.trim().startsWith('theme='));
		if (!theme) {
			return null;
		}
		const value = theme.split('=')[1];
		if (value === 'theme-light' || value === 'theme-dark') {
			return value;
		} else {
			return null;
		}
	};

	const getCurrentThemeFromJS = (): Theme | null => {
		const currentTheme = document.documentElement.dataset.theme;
		if (currentTheme === 'theme-dark' || currentTheme === 'theme-light') {
			return currentTheme;
		}
		return null;
	};

	const getCurrentThemeFromCSS = (): Theme => {
		if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
			return 'theme-dark';
		}
		return 'theme-light';
	};

	const getCurrentTheme = () => {
		const fromCookies = getCurrentThemeFromCookie();
		const fromJS = getCurrentThemeFromJS();
		return fromCookies ? fromCookies : fromJS ? fromJS : getCurrentThemeFromCSS();
	};

	const setCookie = (value: string) => {
		document.cookie = `theme=${value}; expires=Fri, 31 Dec 9999 23:59:59 GMT`;
	};

	let text = '';
	const setTheme = (theme: Theme) => {
		document.documentElement.dataset.theme = theme;
		setCookie(theme);

		if (theme === 'theme-dark') {
			text = 'lighter';
		} else {
			text = 'darker';
		}
	};

	const toggleTheme = () => {
		const currentTheme = getCurrentTheme();
		if (currentTheme === 'theme-dark') {
			setTheme('theme-light');
		} else {
			setTheme('theme-dark');
		}
	};

	onMount(() => {
		setTheme(getCurrentTheme());
	});
	const show = (node: HTMLElement) => node.classList.remove('hidden');
</script>

<svelte:head>
	<script>
		function getCurrentThemeFromCookie() {
			const cookie = document.cookie;
			const theme = cookie.split(';').find((c) => c.trim().startsWith('theme='));
			if (!theme) {
				return null;
			}
			const value = theme.split('=')[1];
			if (value === 'theme-light' || value === 'theme-dark') {
				return value;
			} else {
				return null;
			}
		}

		function getCurrentThemeFromJS() {
			const currentTheme = document.documentElement.dataset.theme;
			if (currentTheme === 'theme-dark' || currentTheme === 'theme-light') {
				return currentTheme;
			}
			return null;
		}

		function getCurrentThemeFromCSS() {
			if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
				return 'theme-dark';
			}
			return 'theme-light';
		}

		function getCurrentTheme() {
			const fromCookies = getCurrentThemeFromCookie();
			const fromJS = getCurrentThemeFromJS();
			return fromCookies ? fromCookies : fromJS;
		}

		function setCookie(value) {
			document.cookie = `theme=${value}; expires=Fri, 31 Dec 9999 23:59:59 GMT`;
		}

		function setTheme(theme) {
			if (!theme) return;
			document.documentElement.dataset.theme = theme;
			setCookie(theme);
		}

		setTheme(getCurrentTheme());
	</script>
</svelte:head>

<button
	class="hidden right-0 top-0 absolute p-1 px-2 opacity-50"
	use:show
	aria-label="Toggle Light and Dark mode"
	on:click|preventDefault={toggleTheme}
>
	{text}
</button>
