<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import { onMount } from 'svelte';
	import { Canvas } from '$lib/Mapper/Canvas';
	import type { Point } from '$lib/Mapper/Geometry';
	import { Backend } from '$lib/Util/Backend';
	import PopupManager from '$lib/Generic/PopupManager.svelte';
	import { Util } from '$lib/Util/Util';
	import ToolbarPopup from '$lib/Mapper/ToolbarPopup.svelte';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import ModalInfo from '$lib/Generic/ModalInfo.svelte';

	//todo improve drawing system: constant canvas dimensions regardless of canvas element size, add zooming + panning

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	let popupManager: PopupManager;

	let paths: Point[][] = [];
	let redoStack: Point[][] = [];
	let tempPath: Point[] | null = null;

	const redraw = () => {
		if (!ctx) return;
		Canvas.Clear(ctx, canvas);
		Canvas.DrawPaths(ctx, paths);
	};

	const resize = () => {
		if (!canvas) return;
		Canvas.Resize(canvas);
		redraw();
	};

	const onMouseDown = (evt: MouseEvent) => {
		tempPath = [];
		tempPath.push(Canvas.GetMousePos(evt, canvas));
	};

	const onMouseMove = (evt: MouseEvent) => {
		if (!ctx || !tempPath) return;

		const lastPoint = tempPath[tempPath.length - 1];
		const point = Canvas.GetMousePos(evt, canvas);
		tempPath.push(point);
		ctx.lineWidth = 3;
		ctx.beginPath();
		ctx.moveTo(lastPoint.x, lastPoint.y);
		ctx.lineTo(point.x, point.y);
		ctx.stroke();
	};

	const onMouseUp = () => {
		if (!tempPath) return;
		paths.push(tempPath);
		tempPath = null;
	};

	onMount(async () => {
		ctx = canvas.getContext('2d');
		paths = await Backend.GetDrawing();
		resize();
	});

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
				type: 'shape',
				location: { x: rect.x, y: rect.y },
				onClear,
				onUndo,
				onRedo,
				onHelp
			})
		);
	};

	const onClear = () => {
		if (!ctx) return;
		Canvas.Clear(ctx, canvas);
		paths.length = 0;
		redoStack.length = 0;
		Backend.SendDrawing(paths);
	};

	const onUndo = () => {
		const path = paths.pop();
		if (!path) return;
		redoStack.push(path);
		redraw();
	};

	const onRedo = () => {
		const path = redoStack.pop();
		if (!path) return;

		paths.push(path);
		redraw();
	};

	const onHelp = () => {
		const id = 'helpPopup';
		if (popupManager.hasPopup(id)) return;

		popupManager.addPopup(
			id,
			Util.ComponentToDom(ModalInfo, {
				id,
				parent: popupManager,
				title: 'Help',
				description: 'Draw the shape of a space <br> Hi'
			})
		);
	};

	const onKeyDown = (evt: KeyboardEvent) => {
		evt.preventDefault();
		if (evt.ctrlKey) {
			if (evt.key === 'z') {
				onUndo();
			} else if (evt.key === 'y') {
				onRedo();
			}
		} else if (evt.key === 'F1') {
			onHelp();
		} else if (evt.key === 'Esc' || evt.key === 'Escape') {
			popupManager.removePopup('helpPopup');
		}
	};
</script>

<svelte:window on:resize={resize} on:keydown={onKeyDown} />
<PopupManager bind:this={popupManager} />

<div>
	<Navbar
		title="Create Space"
		nextLabel="Save"
		handleBack={() => {
			goto('/setup');
		}}
		handleNext={async () => {
			const resp = await Backend.SendDrawing(paths);
			if (!resp) return; //todo toast

			goto('/setup');
		}}
	/>
	<div class="floating">
		<Button
			size="xl"
			type="icon"
			popupText="Toggle Toolbar"
			popupDir="r"
			on:click={onMenuButtonClick}
		>
			<Icon
				viewBox="0 -960 960 960"
				path="M120-240v-80h720v80H120Zm0-200v-80h720v80H120Zm0-200v-80h720v80H120Z"
				size={24}
			/>
		</Button>
	</div>

	<canvas
		bind:this={canvas}
		on:mousedown={onMouseDown}
		on:mousemove={onMouseMove}
		on:mouseup={onMouseUp}
		on:mouseleave={onMouseUp}
	></canvas>
</div>

<style>
	.floating {
		position: absolute;
		left: 0;
		bottom: 0;
		margin: 1em;
	}
</style>
