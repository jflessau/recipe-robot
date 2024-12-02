<script lang="ts">
	import { Api, Ingredient } from './../api.svelte';
	import { onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import { getMe } from './../store.svelte';
	import NotebookText from '~icons/lucide/notebook-text';
	import Basket from '~icons/lucide/shopping-basket';
	import Coins from '~icons/lucide/coins';
	import FileWaring from '~icons/lucide/file-warning';
	import CirclePlus from '~icons/lucide/circle-plus';
	import CircleMinus from '~icons/lucide/circle-minus';
	import CircleCheck from '~icons/lucide/circle-check';
	import Send from '~icons/lucide/send';

	export let ingredient: Ingredient;

	const dispatch = createEventDispatcher();
	let state: 'LOADING' | 'ERROR' | 'IDLE' = 'LOADING';
	$: ingredient = { ...ingredient };
	$: expanded = false;
	$: edit = false;
	$: edited = false;

	const refresh = async () => {
		state = 'LOADING';
		try {
			let r = await Api.ingredientItems(ingredient);
			dispatch('update', r.data);
			getMe();
			state = 'IDLE';
		} catch (e) {
			state = 'ERROR';
			console.error(e);
		}
	};

	onMount(async () => {
		if (!ingredient.item) {
			await refresh();
		}
	});
</script>

<div class="w-full flex bg-mid p-3 rounded-lg ingredient-list-item gap-2 {state === 'LOADING' ? 'pulsating' : ''}">
	{#if ingredient.item}
		<div class="w-full flex-col items-start justify-start gap-1.5">
			<div class="w-full flex flex-row items-center justify-start gap-2">
				<NotebookText class="min-w-5 min-h-5" />
				{#if edit}
					<input class="text-s h-7 rounded border-none px-2 py-1 w bg-bg" bind:value={ingredient.name} />
					<button
						class="rounded bg-info p-1"
						disabled={ingredient.name.trim().length < 2}
						on:click={async () => {
							ingredient.name = ingredient.name.trim();
							ingredient.unit = 'Stück';
							ingredient.quantity = 1;
							ingredient.probablyAtHome = false;
							edit = false;
							edited = true;
							await refresh();
						}}
					>
						<Send class="w-5 h-5 text-color-inverted" />
					</button>
				{:else}
					<button on:click={() => (edit = true)}>
						<p class="text-s">
							{ingredient.name}
						</p>
					</button>
					{#if !edited}
						<p class="opacity-50 text-s">({ingredient.quantity} {ingredient.unit})</p>
					{/if}
				{/if}
			</div>
			<div class="w-full flex flex-row items-center justify-start gap-2">
				<Basket class="min-w-5 min-h-5" />
				<p class="font-bold text-attention text-left pr-2 text-s">
					{#if ingredient.item.url}
						<a href={ingredient.item.url || '#'} target="_blank" rel="noopener noreferrer" class="text-attention">
							{ingredient.item.name}
						</a>
					{:else}
						{ingredient.item.name}
					{/if}
				</p>
			</div>
			{#if ingredient.item.priceCent}
				<div class="w-full flex flex-row items-center justify-start gap-2">
					<Coins class="w-5 h-5" />
					<p class="text-m font-bold">{((ingredient.item.priceCent * ingredient.itemQuantity) / 100).toFixed(2)} €</p>
					<p class="text-s opacity-50">({(ingredient.item.priceCent / 100).toFixed(2)} € / Stück)</p>
				</div>
			{/if}

			<button
				class="underline text-info text-s pt-2"
				on:click={() => {
					expanded = !expanded;
				}}
			>
				{expanded ? 'weniger' : 'mehr'} anzeigen
			</button>
			{#if expanded}
				<div class="w-full flex flex-col items-start justify-start">
					<img src={ingredient.item.imageUrl} alt={ingredient.item.name} class="mt-6 w-48 rounded object-contain" />
				</div>
				{#if ingredient.alternatives}
					<p class="pt-2 w-full text font-bold text-left">Alternativen</p>
				{/if}
				<div class="w-full flex flex-col overflow-hidden rounded bg-bg">
					{#each ingredient.alternatives as a}
						<div class="item clickable bg-bg w-full px-3 py-1 flex flex-row items-center justify-start gap-3">
							{#if a.id === ingredient.item.id}
								<CircleCheck class="w-6 h-6 text-attention" />
							{/if}
							<button
								class="w-full flex flex-col justify-start items-start"
								on:click={() => {
									ingredient.item = a;
									dispatch('update', ingredient);
								}}
							>
								<p class="text-s font-bold text-left text-{a.id === ingredient.item.id ? 'attention' : 'text'}">
									{a.name}
								</p>
								{#if a.priceCent}
									<p class="text-s">{(a.priceCent / 100).toFixed(2)} €</p>
								{/if}
							</button>
						</div>
					{/each}
				</div>
			{/if}
		</div>
		<div class="w-12 flex flex-col">
			<button
				class="w-full p-1 py-2 flex items-center justify-center bg-bg rounded-t"
				on:click={() => {
					ingredient.itemQuantity != null && ingredient.itemQuantity < 99 && ingredient.itemQuantity++;
					dispatch('update', ingredient);
				}}
			>
				<CirclePlus class="w-5 h-5" />
			</button>
			<div class="w-full flex items-center justify-center border-y border-mid">
				<p class="p-1 text-m font-bold bg-bg w-full">{ingredient.itemQuantity}<sub class="pl-0.5">x</sub></p>
			</div>
			<button
				class="w-full px-1 py-2 flex items-center justify-center bg-bg rounded-b"
				on:click={() => {
					ingredient.itemQuantity != null && ingredient.itemQuantity > 0 && ingredient.itemQuantity--;
					dispatch('update', ingredient);
				}}
			>
				<CircleMinus class="w-5 h-5" />
			</button>
		</div>
	{:else}
		<div class="w-full flex-col items-start justify-start gap-2">
			<div class="w-full flex flex-row items-center justify-start gap-2">
				<NotebookText class="w-5 h-5" />
				<p class="text-s">
					{ingredient.name}
				</p>
			</div>
			{#if state === 'LOADING'}
				<div class="w-full flex flex-row items-center justify-start gap-2">
					<Basket class="w-5 h-5" />
					<p class="font-bold text-s text-attention pulsating">Lade...</p>
				</div>
			{:else}
				<div class="w-full flex flex-row items-center justify-start gap-2">
					<FileWaring class="w-5 h-5 text-error" />
					<p class="font-bold text-s text-error">Keine passenden Produkte gefunden</p>
				</div>
			{/if}
		</div>
	{/if}
</div>
