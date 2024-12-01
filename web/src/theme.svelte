<script lang="ts" context="module">
	import { themeStore, Theme } from './store.svelte';
	import { get } from 'svelte/store';

	import t from './style/var.module.scss';

	let themeData: Theme = get(themeStore);

	export function systemThemeIsDark(): boolean {
		return window.matchMedia('(prefers-color-scheme: dark)').matches;
	}

	export let theme = {
		dark: themeData === Theme.DARK,
		background: themeData === Theme.LIGHT ? t.background : t.backgroundDark,
		foreground: themeData === Theme.LIGHT ? t.foreground : t.foregroundDark,
		text: themeData === Theme.LIGHT ? t.text : t.textDark,
		gray: themeData === Theme.LIGHT ? t.gray : t.grayDark,

		backgroundDark: t.backgroundDark,
		foregroundDark: t.foregroundDark,
		textDark: t.textDark,
		grayDark: t.grayDark,

		bismark: t.bismark,
		pancho: t.pancho,
		apricot: t.apricot,
		action: t.action,

		font: t.font
	};
</script>
