<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { Point } from './Geometry';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';

	export let type: string;
	export let location: Point;
	const offsetY = -10;

	let ref: HTMLDivElement;

	//Toolbar functionality
	export let onClear: () => void = () => {};
	export let onUndo: () => void = () => {};
	export let onRedo: () => void = () => {};
	export let onHelp: () => void = () => {};

	onMount(async () => {
		await tick(); // Wait for DOM to finish loading before getting client rect
		const rect = ref.getBoundingClientRect();
		const offset = -rect.height / 2 + offsetY;
		const left = location.x - rect.width / 2;
		const top = location.y - rect.height / 2 + offset;
		const padding = 15;

		const w = window.innerWidth;
		const h = window.innerHeight;

		ref.style.left = `${Math.max(padding, Math.min(w - rect.width - padding, left))}px`;
		ref.style.top = `${Math.max(padding, Math.min(h - rect.height - padding, top))}px`;
	});
</script>

<div class="popup-container" bind:this={ref}>
	<div class="toolbar-layout">
		{#if type === 'shape'}
			<Button size="xl" type="icon" popupText="Clear Drawing" popupDir="r" on:click={onClear}>
				<Icon
					viewBox="0 -960 960 960"
					path="m644-448-56-58 122-94-230-178-94 72-56-58 150-116 360 280-196 152Zm115 114-58-58 73-56 66 50-81 64Zm33 258L632-236 480-118 120-398l66-50 294 228 94-73-57-56-37 29-360-280 83-65L55-811l57-57 736 736-56 56ZM487-606Z"
					size={24}
				/>
			</Button>
			<Button size="xl" type="icon" popupText="Undo (Ctrl+Z)" popupDir="r" on:click={onUndo}>
				<Icon
					viewBox="0 -960 960 960"
					path="M280-200v-80h284q63 0 109.5-40T720-420q0-60-46.5-100T564-560H312l104 104-56 56-200-200 200-200 56 56-104 104h252q97 0 166.5 63T800-420q0 94-69.5 157T564-200H280Z"
					size={24}
				/>
			</Button>
			<Button size="xl" type="icon" popupText="Redo (CTRL+Y)" popupDir="r" on:click={onRedo}>
				<Icon
					viewBox="0 -960 960 960"
					path="M396-200q-97 0-166.5-63T160-420q0-94 69.5-157T396-640h252L544-744l56-56 200 200-200 200-56-56 104-104H396q-63 0-109.5 40T240-420q0 60 46.5 100T396-280h284v80H396Z"
					size={24}
				/>
			</Button>
		{/if}
		<Button size="xl" type="icon" popupText="Help (F1)" popupDir="r" on:click={onHelp}>
			<Icon
				viewBox="0 -960 960 960"
				path="M478-240q21 0 35.5-14.5T528-290q0-21-14.5-35.5T478-340q-21 0-35.5 14.5T428-290q0 21 14.5 35.5T478-240Zm-36-154h74q0-33 7.5-52t42.5-52q26-26 41-49.5t15-56.5q0-56-41-86t-97-30q-57 0-92.5 30T342-618l66 26q5-18 22.5-39t53.5-21q32 0 48 17.5t16 38.5q0 20-12 37.5T506-526q-44 39-54 59t-10 73Zm38 314q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"
				size={24}
			/>
		</Button>
	</div>
</div>

<style>
	.popup-container {
		position: absolute;
		background-color: transparent;
	}
	.toolbar-layout {
		display: flex;
		flex-direction: column;
	}
</style>
