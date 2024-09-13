<script lang="ts">
	import Button from './Button.svelte';

	export let title: string = '';
	export let backLabel: string = 'Back';
	export let nextLabel: string = 'Next';
	export let handleBack: (() => void) | null = null;
	export let handleNext: (() => void) | null = null;
</script>

<div class="nav-container">
	<Button on:click={handleBack} padding="4px" bg="transparent" disabled={handleBack === null}
		>{backLabel}</Button
	>
	<h1>{title}</h1>
	<Button on:click={handleNext} padding="4px" bg="transparent" disabled={handleNext === null}
		>{nextLabel}</Button
	>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.nav-container {
		width: 100%;
		height: 40px;
		background: #eee;
		border-bottom: 2px solid rgba(0, 0, 0, 0.2);
		display: flex;
		justify-content: space-between;
		align-items: center;
		overflow: hidden;
		padding: 0px 8px;
	}
	h1 {
		margin: 0;
		height: max-content;
		font-size: medium;
		user-select: none;
		font-family: 'Figtree';
	}
</style>
