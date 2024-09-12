<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';

	export let name;
	export let path;

	const handleEdit = () => {
		goto(`/stores/${name}`);
	};
</script>

<div class="row-container">
	<div class="text-container">
		<h3>{name}</h3>
		<span>{path}</span>
	</div>

	<Button on:click={handleEdit}>
		<Icon
			viewBox="0 -960 960 960"
			path="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"
			size={24}
		/>
	</Button>
</div>

<style>
	.row-container {
		display: flex;
		justify-content: space-between;
		padding: 6px;
	}
	.text-container {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	h3 {
		margin: 0;
		padding: 0;
		font-family: 'Figtree';
	}
	span {
		margin: 0;
		padding: 0;
		font-family: 'Calibri';
	}
</style>
