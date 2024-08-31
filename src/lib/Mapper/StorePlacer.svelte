<script lang="ts">
	import { onMount } from 'svelte';
	import type { Point } from './Geometry';
	import { Canvas } from './Canvas';
	import { Backend } from '$lib/Util/Backend';
	import PopupManager from '$lib/Generic/PopupManager.svelte';
	import StorePopup from './StorePopup.svelte';
	import { Util } from '$lib/Util/Util';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import ToolbarPopup from './ToolbarPopup.svelte';
	import ModalPopup from '$lib/Generic/ModalPopup.svelte';
	import type { Store } from '$lib/Util/Models';

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	let popupManager: PopupManager;

	let paths: Point[][] = [];
	let stores: Store[] = [];

	let tempCoord: Point | null;
	let isPlacing = true;

	onMount(async () => {
		paths = await Backend.GetDrawing();
		stores = await Backend.FetchAllStores(1);
		ctx = canvas.getContext('2d');
		resize();
	});

	const redraw = () => {
		if (!ctx) return;
		Canvas.Clear(ctx, canvas);
		Canvas.DrawPaths(ctx, paths);

		for (const store of stores) {
			Canvas.DrawCircle(ctx, { x: store.location.x, y: store.location.y }, 10, 'orange');
		}

		if (tempCoord) {
			Canvas.DrawCircle(ctx, tempCoord, 10, 'green');
		}
	};

	const handleMouseMove = (e: MouseEvent) => {
		const mousePos = Canvas.GetMousePos(e, canvas);
		if (isPlacing) tempCoord = { x: mousePos.x, y: mousePos.y };
		redraw();
	};

	const handleMouseClick = (e: MouseEvent) => {
		if (!ctx || !tempCoord) return;
		isPlacing = false;

		Canvas.DrawCircle(ctx, tempCoord, 10, 'green');

		const id = 'storePopup';
		if (popupManager.hasPopup(id)) popupManager.removePopup(id);

		popupManager.addPopup(
			id,
			Util.ComponentToDom(StorePopup, { id, parent: popupManager, location: tempCoord })
		);
	};

	const resize = () => {
		if (!canvas) return;
		Canvas.Resize(canvas);
		redraw();
	};

	const onMenuButtonClick = (evt: MouseEvent) => {
		const ele = evt.target as HTMLButtonElement;
		if (!ele) return;

		const rect = ele.getBoundingClientRect();

		const id = 'toolbarPopup';
		if (popupManager.hasPopup(id)) {
			popupManager.removePopup(id);
			return;
		}

		popupManager.addPopup(
			id,
			Util.ComponentToDom(ToolbarPopup, {
				type: 'place',
				location: { x: rect.x, y: rect.y },
				onHelp
			})
		);
	};

	const onHelp = () => {
		const id = 'helpPopup';
		if (popupManager.hasPopup(id)) return;

		popupManager.addPopup(
			id,
			Util.ComponentToDom(ModalPopup, {
				id,
				parent: popupManager,
				title: 'Help',
				description: 'Draw the shape of a space <br> Hi'
			})
		);
	};
</script>

<svelte:window on:resize={resize} />
<PopupManager
	bind:this={popupManager}
	onRemovePopup={(id) => {
		if (id === 'storePopup') isPlacing = true;
	}}
/>

<canvas bind:this={canvas} on:mousemove={handleMouseMove} on:click={handleMouseClick} />
<div class="floating">
	<Button size="xl" type="icon" on:click={onMenuButtonClick}>
		<Icon
			viewBox="0 -960 960 960"
			path="M120-240v-80h720v80H120Zm0-200v-80h720v80H120Zm0-200v-80h720v80H120Z"
			size={24}
		/>
	</Button>
</div>

<style>
	.floating {
		position: absolute;
		left: 0;
		bottom: 0;
		margin: 1em;
	}
</style>
