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
	<div class="w-full flex flex-col justify-start items-center gap-8 py-8">
		<img
			src="/img/logo.png"
			alt="shopping bag with various items like apples, bottles, fruits and vegetables"
			class="w-40"
		/>
		{#if me}
			<Route path="/" component={Home} />
			<Route path="/join" component={Home} />
			<Route path="/login" component={Home} />
		{:else}
			<Route path="/" component={Login} />
			<Route path="/join" component={Join} />
		{/if}
	</div>
</Router>
