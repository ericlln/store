<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import type { Store } from './Util/Models';

	export let store: Store;

	const handleView = () => {
		goto(`/stores/${store.name}`);
	};
</script>

<Button
	on:click={handleView}
	padding="10px"
	type="space-between"
	disabled={!store.available}
	popupText={!store.available
		? 'Not avaliable due to file being moved, deleted, or held open by another application. '
		: ''}
	popupDir="r"
>
	<div class="text-container">
		<h3>{store.name}</h3>
		<span>{store.path}</span>
	</div>
	<Icon
		viewBox="0 -960 960 960"
		path="M480-320q75 0 127.5-52.5T660-500q0-75-52.5-127.5T480-680q-75 0-127.5 52.5T300-500q0 75 52.5 127.5T480-320Zm0-72q-45 0-76.5-31.5T372-500q0-45 31.5-76.5T480-608q45 0 76.5 31.5T588-500q0 45-31.5 76.5T480-392Zm0 192q-146 0-266-81.5T40-500q54-137 174-218.5T480-800q146 0 266 81.5T920-500q-54 137-174 218.5T480-200Zm0-300Zm0 220q113 0 207.5-59.5T832-500q-50-101-144.5-160.5T480-720q-113 0-207.5 59.5T128-500q50 101 144.5 160.5T480-280Z"
		size={24}
	/>
</Button>

<style>
	.text-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		text-align: left;
		gap: 6px;
		overflow: hidden;
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
		white-space: normal;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}
</style>
