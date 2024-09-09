<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import SpaceRow from '$lib/SpaceRow.svelte';
	import { Backend } from '$lib/Util/Backend';
	import type { Space } from '$lib/Util/Models';
	import { onMount } from 'svelte';

	const storeName = $page.params.slug;

	let spaces: Space[] | null = null;

	onMount(async () => {
		spaces = await Backend.GetSpaces(storeName);
	});
</script>

<div class="layout-wrapper">
	<span>Store: {storeName}</span>
	{#if spaces}
		<div class="spaces-list">
			{#each spaces as space (space.id)}
				<SpaceRow {space} {storeName} />
			{/each}
		</div>
	{/if}
	<Button
		on:click={async () => {
			await goto(`/store/${storeName}/spaces/creator`);
		}}>Create New Space</Button
	>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.layout-wrapper {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 20px;
	}
	.spaces-list {
		width: 90%;
		display: flex;
		flex-direction: column;
		flex: 1;
		overflow-y: auto;
		gap: 10px;
		padding: 12px;
		background-color: blueviolet;
		border-radius: 12px;
	}
</style>
