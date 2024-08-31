<script lang="ts">
	export let value: string = '';
	export let placeholder: string = '';
	export let label: string = '';
	export let id: string = '';
	export let rows: number = 5;
	export let cols: number = 30;
	export let height: number = 30;

	let ref: HTMLTextAreaElement;
	let fontSize = 0.45 * height;
</script>

<div>
	{#if label}
		<label for={id}>{label}</label>
	{/if}
	<textarea
		bind:this={ref}
		bind:value
		{placeholder}
		{rows}
		{cols}
		style="height: {height}px; font-size: {fontSize}px"
		{id}
	/>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	div {
		padding: 4px;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	label {
		margin-bottom: 4px;
		font-family: 'Figtree';
	}
	textarea {
		border: 1px solid #ccc;
		padding: 16px 10px;
		border-radius: 4px;
		font-family: 'Inter';
		resize: none;
	}
</style>
