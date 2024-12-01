<script lang="ts">
	import LoadingSpinner from '../components/LoadingSpinner.svelte';
	import Error from '../components/Error.svelte';
	import { navigate } from 'svelte-routing';
	import { Api } from './../api.svelte';

	let credetials: { username: string; password: string } | null = null;
	let state: 'IDLE' | 'LOADING' | 'ERROR' | 'SUCCESS' = 'IDLE';
	$: state = state;

	$: inviteCode = '';
</script>

<div class="w-full flex flex-col items-center justify-start gap-8">
	<h1 class="w-full text-center text-2xl font-black text-attention">
		{state === 'SUCCESS' ? 'Registrierung erfolgreich!' : 'Registrieren'}
	</h1>

	{#if state === 'ERROR'}
		<Error title="Der Einladungscode ist ungültig." />
		<button
			class="fancy"
			on:click={() => {
				state = 'IDLE';
				inviteCode = '';
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
					let r = await Api.join(inviteCode);
					if (r.status === 200) {
						credetials = r.data;
						state = 'SUCCESS';
					} else {
						state = 'ERROR';
						console.error(`join failed with status: ${r.status}`);
					}
				} catch (e) {
					state = 'ERROR';
					console.error(e);
				}
			}}
		>
			<div class="w-full flex flex-col gap-1 text-s font-bold">
				<label for="invite-code-input" class="text-center">Einladungscode</label>
				<input
					id="invite-code-input"
					type="text"
					class="text-center"
					placeholder="Einladungscode"
					bind:value={inviteCode}
				/>
			</div>
			<button class="fancy">Los geht's!</button>

			<p class="text-s font-bold mt-16">
				Du hast schon einen Account? <a href="/" class="text-attention">Hier einloggen</a>
			</p>
		</form>
	{/if}

	{#if state === 'SUCCESS' && credetials !== null}
		<div class="w-full flex flex-col items-center justify-start gap-8">
			<p class="text-center">Hier sind deine Zugangsdaten</p>
			<p class="text-center">Benutzername:<br /><span class="font-bold text-m">{credetials.username}</span></p>
			<p class="text-center">Passwort:<br /><span class="font-bold text-m">{credetials.password}</span></p>
			<p class="text-center">Speicher deine Zugangsdaten JETZT ab.<br />Das ist das einzige Mal, dass du sie siehst.</p>
			<button
				class="fancy"
				on:click={() => {
					navigate('/');
				}}
			>
				Zum Login
			</button>
		</div>
	{/if}
</div>
