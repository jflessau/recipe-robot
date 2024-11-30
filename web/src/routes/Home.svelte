<script lang="ts">
	import LoadingSpinner from '../components/LoadingSpinner.svelte';
	import Error from '../components/Error.svelte';
	import { Api, Ingredient, Item } from './../api.svelte';
	import NotebookPen from '~icons/lucide/notebook-pen';
	import NotebookText from '~icons/lucide/notebook-text';
	import Basket from '~icons/lucide/shopping-basket';
	import Coins from '~icons/lucide/coins';
	import FileWaring from '~icons/lucide/file-warning';
	import CirclePlus from '~icons/lucide/circle-plus';
	import CircleMinus from '~icons/lucide/circle-minus';
	import Banknote from '~icons/lucide/banknote';
	import CheckCircle from '~icons/lucide/check-circle';

	$: recipe = '';
	let state: 'IDLE' | 'LOADING' | 'ERROR' | Ingredient[] = 'IDLE';
	// $: state = state;

	$: state = [
		{
			id: '1',
			name: 'Tomate',
			probablyAtHome: true,
			unit: 'Stück',
			quantity: 2,
			item: {
				id: '1',
				name: 'Tomate',
				quantity: 2,
				priceCent: 100,
				url: 'https://www.google.com',
				imageUrl: 'https://jflessau.com/img/placeholder.jpg'
			},
			itemQuantity: 1,
			alternatives: [
				{
					id: '2',
					name: 'Tomate',
					quantity: 2,
					priceCent: 100,
					url: 'https://www.google.com',
					imageUrl: 'https://jflessau.com/img/placeholder.jpg'
				},
				{
					id: '2',
					name: 'Cherrytomaten',
					quantity: 2,
					priceCent: 100,
					url: 'https://www.google.com',
					imageUrl: 'https://jflessau.com/img/placeholder.jpg'
				}
			]
		},
		{
			id: '2',
			name: 'Tomate',
			probablyAtHome: true,
			unit: 'Stück',
			quantity: 0,
			item: null,
			itemQuantity: 1,
			alternatives: []
		}
	];
</script>

<div class="w-full flex flex-col items-center justify-start gap-8">
	<img
		src="/img/logo.png"
		alt="shopping bag with various items like apples, bottles, fruits and vegetables"
		class="w-32"
	/>
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

		<h1 class="w-full text-center text-xl font-black">Deine Einkaufsliste</h1>

		<div class="w-full flex flex-col items-start justify-start gap-2">
			{#each state as i}
				<div class="w-full flex bg-mid p-3 rounded-lg ingredient-list-item">
					{#if i.item}
						<div class="w-full flex-col items-start justify-start gap-1.5">
							<div class="w-full flex flex-row items-center justify-start gap-3">
								<NotebookText class="w-5 h-5" />
								<p class="text-s">
									{i.name}
								</p>
								<p class="opacity-50 text-s">({i.quantity} {i.unit})</p>
							</div>
							<div class="w-full flex flex-row items-center justify-start gap-2">
								<Basket class="w-6 h-6" />
								<p class="font-bold text-attention">
									{#if i.item.url}
										<a href={i.item.url || '#'} target="_blank" rel="noopener noreferrer" class="text-attention">
											{i.item.name}
										</a>
									{:else}
										{i.item.name}
									{/if}
								</p>
							</div>
							{#if i.item.priceCent && i.item.quantity}
								<div class="w-full flex flex-row items-center justify-start gap-3">
									<Coins class="w-5 h-5" />
									<p class="text-m font-bold">{((i.item.priceCent * i.item.quantity) / 100).toFixed(2)} €</p>
									<p class="text-s opacity-50">({(i.item.priceCent / 100).toFixed(2)} € / Stück)</p>
								</div>
							{/if}
							<button class="underline text-info text-s pt-2">mehr infos</button>
							<div class="w-full flex flex-col items-start justify-start">
								<img src={i.item.imageUrl} alt={i.item.name} class="w-48 rounded object-contain" />
							</div>
							{#if i.alternatives}
								<p class="pt-2 w-full text font-bold text-left">Alternativen</p>
							{/if}
							<div class="w-full flex flex-col overflow-hidden rounded bg-bg">
								{#each i.alternatives as a}
									<div class="item clickable bg-bg w-full px-3 py-1 flex flex-row items-center justify-start gap-3">
										{#if (i.item.id = a.id)}
											<CheckCircle class="w-5 h-5 text-success" />
										{/if}
										<button class="w-full flex flex-col justify-start items-start">
											<p class="text-s font-bold">{a.name}</p>
											{#if a.priceCent}
												<p class="text-s">{(a.priceCent / 100).toFixed(2)} €</p>
											{/if}
										</button>
									</div>
								{/each}
							</div>
						</div>
						<div class="w-12 flex flex-col">
							<button class="w-full p-1 py-2 flex items-center justify-center bg-bg rounded-t">
								<CirclePlus class="w-5 h-5" />
							</button>
							<div class="w-full flex items-center justify-center border-y border-mid">
								<p class="p-1 text-m font-bold bg-bg w-full">{i.item.quantity}<sub class="pl-0.5">x</sub></p>
							</div>
							<button class="w-full px-1 py-2 flex items-center justify-center bg-bg rounded-b">
								<CircleMinus class="w-5 h-5" />
							</button>
						</div>
					{:else}
						<div class="w-full flex-col items-start justify-start gap-2">
							<div class="w-full flex flex-row items-center justify-start gap-2">
								<NotebookText class="w-5 h-5" />
								<p class="text-s">
									{i.name}
								</p>
							</div>
							<div class="w-full flex flex-row items-center justify-start gap-2">
								<FileWaring class="w-5 h-5 text-error" />
								<p class="font-bold text-s text-error">Keine passenden Produkte gefunden</p>
							</div>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
