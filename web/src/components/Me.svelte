<script lang="ts">
	import { meStore, getMe } from './../store.svelte';

	$: me = $meStore;
</script>

{#if me}
	<div class="w-full flex flex-col justify-center items-center gap-2">
		<p class="text-s text-center w-full">
			Dein Nutzername:
			<span class="font-black">{me.username}</span>
		</p>

		{#if me.percentageOfDailyLimit < 95}
			<p class="text-s text-center w-full">
				Tägliches Limit für Anfragen: <span class="font-black">{me.percentageOfDailyLimit} %</span>
			</p>
			<p class="text-xs">
				Die künstliche Intelligenz, die Rezepte und Zutaten verarbeitet kostet Geld. Und weil ich davon nicht unendlich
				viel habe, gibt es ein tägliches Limit.
			</p>
		{:else}
			<p class="text-m font-black text-center text-error w-full">
				Du hast dein tägliches Limit für Anfragen erreicht. Du kannst morgen weitermachen :)
			</p>
		{/if}
	</div>
{/if}
