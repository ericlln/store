<script lang="ts">
	import Button from './Button.svelte';

	export let title: string = '';
	export let backLabel: string = 'Back';
	export let nextLabel: string = 'Next';
	export let handleBack: (() => void) | null = null;
	export let handleNext: (() => void) | null = null;
</script>

<div class="nav-container">
	{#if handleBack}
		<Button on:click={handleBack} disabled={handleBack === null}>{backLabel}</Button>
	{/if}

	<h1>{title}</h1>

	{#if handleNext}
		<Button on:click={handleNext} disabled={handleNext === null}>{nextLabel}</Button>
	{/if}
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.nav-container {
		width: 100%;
		height: 34px;
		background: #eee;
		display: flex;
		justify-content: space-between;
		align-items: center;
		overflow: hidden;
		padding: 0px 4px;
	}
	h1 {
		margin: 0;
		height: max-content;
		font-size: medium;
		user-select: none;
		font-family: 'Figtree';
	}
</style>
