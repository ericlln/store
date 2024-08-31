<script lang="ts">
	import type { Point } from '$lib/Mapper/Geometry';
	import { onMount, tick } from 'svelte';
	import PopupManager from '$lib/Generic/PopupManager.svelte';
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';

	const offsetY = 10;

	export let id: string;
	export let parent: PopupManager;
	export let location: Point;

	let ref: HTMLDivElement;

	onMount(async () => {
		await tick(); // Wait for DOM to finish loading before getting client rect
		const rect = ref.getBoundingClientRect();
		const offset = -rect.height / 2 + offsetY;
		const left = location.x - rect.width / 2;
		const top = location.y - rect.height / 2 + offset;
		const padding = 5;

		const w = window.innerWidth;
		const h = window.innerHeight;

		ref.style.left = `${Math.max(padding, Math.min(w - rect.width - padding, left))}px`;
		ref.style.top = `${Math.max(padding, Math.min(h - rect.height - padding, top))}px`;
	});
</script>

<div class="popup-container" bind:this={ref}>
	<Button
		on:click={() => {
			const queryParams = new URLSearchParams({
				x: location.x.toString(),
				y: location.y.toString()
			});
			goto(`/store/editor?${queryParams}`);
		}}>Setup</Button
	>
	<Button
		on:click={() => {
			parent.removePopup(id);
		}}
		>Cancel
	</Button>
</div>

<style>
	.popup-container {
		width: 8em;
		display: flex;
		flex-direction: column;
		gap: 4px;
		position: absolute;
		padding: 4px;
		border-radius: 4px;
		z-index: 50;
		background-color: black;
		color: white;
		pointer-events: none;
	}
</style>
