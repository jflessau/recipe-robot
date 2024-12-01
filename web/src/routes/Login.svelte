<script lang="ts">
	import LoadingSpinner from '../components/LoadingSpinner.svelte';
	import Error from '../components/Error.svelte';
	import { navigate } from 'svelte-routing';
	import { Api } from './../api.svelte';
	import { getMe } from './../store.svelte';

	let state: 'IDLE' | 'LOADING' | 'ERROR' | 'SUCCESS' = 'IDLE';
	$: state = state;

	$: username = '';
	$: password = '';

	$: if (state === 'SUCCESS') {
		getMe();
		navigate('/');
	}
</script>

<div class="w-full flex flex-col items-center justify-start gap-8">
	<h1 class="w-full text-center text-2xl font-black text-attention">Login</h1>

	{#if state === 'ERROR'}
		<Error title="Die Anmeldedaten sind ungültig." />
		<button
			class="fancy"
			on:click={() => {
				state = 'IDLE';
				username = '';
				password = '';
			}}>Nochmal versuchen</button
		>
	{/if}

	{#if state === 'LOADING'}
		<LoadingSpinner />
	{/if}

	{#if state === 'IDLE'}
		<form
			class="w-full flex flex-col items-center justify-start gap-8"
			on:submit|preventDefault={async () => {
				state = 'LOADING';
				try {
					await Api.login({
						username,
						password
					});
					state = 'SUCCESS';
				} catch (e) {
					state = 'ERROR';
					console.error(e);
				}
			}}
		>
			<div class="w-full flex flex-col gap-1 text-s font-bold">
				<label for="username-input" class="text-center">Benutzername</label>
				<input id="username-input" type="text" class="text-center" placeholder="Benutzername" bind:value={username} />
			</div>

			<div class="w-full flex flex-col gap-1 text-s font-bold">
				<label for="password-input" class="text-center">Passwort</label>
				<input id="password-input" type="password" class="text-center" placeholder="Passwort" bind:value={password} />
			</div>

			<button class="fancy">Los geht's!</button>

			<p class="text-s font-bold mt-16">
				Noch keinen Account?
				<a href="/join" class="text-attention">Registrieren</a>
			</p>
		</form>
	{/if}
</div>
