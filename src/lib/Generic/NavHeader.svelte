<script lang="ts">
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';

	export let title: string;
	export let onBack: () => void = () => {
		history.back();
	};
</script>

<div class="layout-wrapper">
	<Button on:click={onBack} type="centered" fontSize="m">
		<Icon
			viewBox="0 -960 960 960"
			path="M400-80 0-480l400-400 71 71-329 329 329 329-71 71Z"
			size={16}
		/>
		<span>Back</span></Button
	>
	<h2>{title}</h2>
</div>

<style>
	.layout-wrapper {
		width: 100%;
		height: 34px;
		display: flex;
		justify-content: left;
		align-items: center;
		gap: 12px;
	}

	h2 {
		font-family: 'Figtree';
		font-size: larger;
	}
</style>
