<script lang="ts">
	export let type: string = 'text';
	export let value: string;
	export let placeholder: string = '';
	export let maxlength: number = 100; // 0 => no limit
	export let height: number = 30;
	export let width: number = 200;

	let ref: HTMLInputElement;
	let fontSize = 0.45 * height;

	$: if (ref) {
		ref.setAttribute('type', type);
	}
</script>

<input
	bind:this={ref}
	bind:value
	{placeholder}
	{maxlength}
	style="width: {width}px; height: {height}px; font-size: {fontSize}px"
/>

<style>
	* {
		box-sizing: border-box;
	}
	input {
		border: 1px solid #ccc;
		padding: 14px 8px;
		border-radius: 4px;
		font-family: 'Inter';
	}
</style>
