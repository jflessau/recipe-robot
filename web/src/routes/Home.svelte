<script lang="ts">
	import LoadingSpinner from '../components/LoadingSpinner.svelte';
	import Ingredient from '../components/Ingredient.svelte';
	import Error from '../components/Error.svelte';
	import Me from '../components/Me.svelte';
	import { Api, Ingredient as IngredientType } from './../api.svelte';
	import { getMe } from './../store.svelte';
	import NotebookPen from '~icons/lucide/notebook-pen';
	import Trash2 from '~icons/lucide/trash-2';

	$: recipe = '';
	let state: 'IDLE' | 'LOADING' | 'ERROR' | IngredientType[] = 'IDLE';
	$: state = typeof state === 'string' ? state : [...state];
	$: totalCent =
		typeof state === 'string' ? 0 : state.reduce((acc, i) => acc + (i.item?.priceCent || 0) * i.itemQuantity, 0);
</script>

<div class="w-full flex flex-col items-center justify-start gap-8">
	{#if typeof state === 'string'}
		<h1 class="w-full text-center text-2xl font-black text-attention">Rezept Ranger</h1>
	{/if}

	{#if state === 'LOADING'}
		<LoadingSpinner />
	{/if}

	{#if state === 'ERROR'}
		<Error title="Das hat nicht geklappt." />
		<button
			class="fancy"
			on:click={() => {
				state = 'IDLE';
			}}>Nochmal versuchen</button
		>
	{/if}

	{#if state === 'IDLE'}
		<form
			class="w-full flex flex-col items-center justify-start gap-8"
			on:submit|preventDefault={async () => {
				state = 'LOADING';
				try {
					let r = await Api.recipeIngredients(recipe);
					state = r.data.ingredients;
					getMe();
				} catch (e) {
					state = 'ERROR';
					console.error(e);
				}
			}}
		>
			<div class="w-full flex flex-col gap-1 text-s">
				<label for="recipe-input" class="text-left font-bold">Rezept eingeben</label>

				<textarea id="recipe-input" bind:value={recipe} class="w-full text-s" placeholder="Hier Rezept eingeben..."
				></textarea>
			</div>
			{#if recipe.trim().length > 0}
				<button class="fancy"> Zutaten finden! </button>
			{/if}
		</form>
	{/if}

	{#if typeof state !== 'string'}
		<button
			class="flex flex-row items-center justify-center gap-2 px-2 py-1 text-info text-s border border-info rounded font-bold"
			on:click={() => {
				state = 'IDLE';
			}}
		>
			<NotebookPen class="w-4 h-4 text-info" />
			Rezept ändern
		</button>

		{#if state.filter((i) => i.probablyAtHome).length > 0}
			<div class="w-full flex flex-col justify-center items-center gap-4">
				<p class="max-w-96 text-s font-bold">
					Diese Zutaten hast du wahrscheinlich schon zu Hause. Klicke sie an um sie zu entfernen.
				</p>
				<div class="w-full flex flex-row flex-wrap items-center justify-center gap-3">
					{#each state.filter((i) => i.probablyAtHome) as i}
						<button
							class="px-2 py-1 text-s text-bold text-error gap-2 rounded border border-error flex items-center justify-center"
							on:click={() => {
								if (typeof state !== 'string') {
									state = state.filter((s) => s.id !== i.id);
								}
							}}
						>
							<Trash2 class="w-5 h-5 text-success" />
							{i.name}
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<h1 class="w-full text-center text-xl font-black">Deine Einkaufsliste</h1>

		{#if state.length === 0}
			<p class="text-center text-s font-bold">...ist leer</p>
		{:else}
			<p class="font-bold text-m">{(totalCent / 100).toFixed(2)} €</p>
			<div class="w-full flex flex-col items-start justify-start gap-2">
				{#each state as i}
					<Ingredient
						ingredient={i}
						on:update={(e) => {
							if (typeof state !== 'string') {
								state = state.map((s) => {
									if (s.id === e.detail.id) {
										return e.detail;
									}
									return s;
								});
							}
						}}
					/>
				{/each}
			</div>
		{/if}
	{/if}

	<Me />
</div>
