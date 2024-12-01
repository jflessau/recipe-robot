<script lang="ts">
	import './style/main.scss';

	import { Router, Route } from 'svelte-routing';
	import Home from './routes/Home.svelte';
	import Join from './routes/Join.svelte';
	import Login from './routes/Login.svelte';
	import { onMount } from 'svelte';
	import { meStore, getMe } from './store.svelte';

	$: me = $meStore;

	onMount(() => {
		getMe();
	});
</script>

<Router>
	<div class="w-full flex flex-col justify-start items-center gap-16 py-8">
		{#if me}
			<Route path="/" component={Home} />
		{:else}
			<Route path="/" component={Login} />
			<Route path="/join" component={Join} />
		{/if}
	</div>
</Router>
