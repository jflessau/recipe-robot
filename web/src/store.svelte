<script lang="ts" context="module">
	import { writable } from 'svelte/store';
	import type { Writable } from 'svelte/store';
	import { Api, Me } from './api.svelte';

	export const meStore: Writable<Me | null> = writable(null);

	// reusable function to get me from api

	export async function getMe(): Promise<void> {
		try {
			let r = await Api.me();
			if (r.status === 200) {
				meStore.set(r.data);
			} else {
				console.error(`getMe failed with status: ${r.status}`);
			}
		} catch (e) {
			console.error(e);
		}
	}
</script>
