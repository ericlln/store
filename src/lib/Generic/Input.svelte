<script lang="ts">
	export let type: string = 'text';

	export let value: string;
	export let placeholder: string = '';
	export let label: string = '';
	export let id: string = '';
	export let maxlength: number = 100; // 0 => no limit
	export let height: number = 30;

	let ref: HTMLInputElement;
	let fontSize = 0.45 * height;

	$: if (ref) {
		ref.setAttribute('type', type);
	}
</script>

<div>
	{#if label}
		<label for={id}>{label}</label>
	{/if}
	<input
		bind:this={ref}
		bind:value
		{placeholder}
		{maxlength}
		style="height: {height}px; font-size: {fontSize}px"
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
	input {
		border: 1px solid #ccc;
		padding: 16px 10px;
		border-radius: 4px;
		font-family: 'Inter';
	}
</style>
