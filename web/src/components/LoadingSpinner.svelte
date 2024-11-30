<script lang="ts">
	export let title: string | undefined = 'Bitte warten...';
</script>

<div class="w-full flex flex-col justify-start items-center gap-8">
	<img src="/img/loading-animation.gif" alt="loading spinner" class="w-24" />
	{#if title}
		<p class="font-bold text-attention text-l">{title}</p>
	{/if}
</div>
