<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import Divider from '$lib/Generic/Divider.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import NavHeader from '$lib/Generic/NavHeader.svelte';
	import PopupManager from '$lib/Generic/PopupManager.svelte';
	import { Canvas } from '$lib/Mapper/Canvas';
	import type { Point } from '$lib/Mapper/Geometry';
	import { Backend } from '$lib/Util/Backend';
	import type { Space } from '$lib/Util/Models';
	import { onMount, tick } from 'svelte';

	const storeName = $page.params.slug;
	const spaceId = $page.params.spaceId;
	let space: Space;

	let popupManager: PopupManager;

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	let tempCoord: Point | null;
	let selectedCoord: Point | null;
	let isPlacing = false;

	onMount(async () => {
		const resp = await Backend.GetSpace(storeName, parseInt(spaceId));
		if (resp) space = resp;

		await tick();
		ctx = canvas.getContext('2d');
		if (ctx) {
			Canvas.FixScaling(ctx, canvas);
			if (space.drawing) Canvas.DrawPaths(ctx, space.drawing);
		}
	});

	const redraw = () => {
		if (!ctx) return;

		Canvas.Clear(ctx, canvas);
		Canvas.FixScaling(ctx, canvas);
		if (space.drawing) Canvas.DrawPaths(ctx, space.drawing);
		if (tempCoord) Canvas.DrawCircle(ctx, tempCoord, 10);
		if (selectedCoord) Canvas.DrawCircle(ctx, selectedCoord, 10);
	};

	const handleMouseMove = (e: MouseEvent) => {
		const mousePos = Canvas.GetMousePos(e, canvas);
		if (isPlacing) tempCoord = { x: mousePos.x, y: mousePos.y };
		redraw();
	};

	const handleClick = () => {
		if (!isPlacing && !tempCoord) return;
		selectedCoord = tempCoord;
		tempCoord = null;
	};

	const stopPlacing = () => {
		tempCoord = null;
		selectedCoord = null;
		redraw();
	};
</script>

{#if space}
	<PopupManager bind:this={popupManager} onRemovePopup={(id) => {}} />

	<div class="layout-wrapper">
		<NavHeader
			title={`Overview of ${space.name}`}
			onBack={() => {
				goto(`/store/${storeName}/spaces`);
			}}
		/>
		<div class="main-content">
			<div class="bin-list">
				<Button
					on:click={() => {
						if (isPlacing) stopPlacing();
						isPlacing = !isPlacing;
					}}
				>
					<Icon
						viewBox="0 -960 960 960"
						path="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"
						size={24}
					/>
				</Button>

				{#if isPlacing}
					<div class="selection-viewer">
						<span
							>{`Selected ${selectedCoord?.x.toFixed(1) ?? 'N/A'}, ${selectedCoord?.y.toFixed(1) ?? 'N/A'}`}</span
						>
						<div>
							<Button
								on:click={() => {
									stopPlacing();
									isPlacing = false;
								}}
							>
								<Icon
									viewBox="0 -960 960 960"
									path="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
									size={18}
								/>
							</Button>
							<Button
								on:click={() => {
									if (!selectedCoord) return;

									const queryParams = new URLSearchParams({
										x: selectedCoord.x.toString(),
										y: selectedCoord.y.toString()
									});
									goto(`/store/${storeName}/spaces/${spaceId}/bins/creator/?${queryParams}`);
								}}
								disabled={selectedCoord === null}
							>
								<Icon
									viewBox="0 -960 960 960"
									path="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"
									size={18}
								/>
							</Button>
						</div>
					</div>
				{/if}

				<Divider />
			</div>

			<div class="canvas-container">
				<canvas bind:this={canvas} on:mousemove={handleMouseMove} on:click={handleClick} />
			</div>
		</div>
	</div>
{/if}

<style>
	* {
		box-sizing: border-box;
	}
	.layout-wrapper {
		width: 90%;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		margin: auto;
		gap: 30px;
		padding: 20px;
	}
	.main-content {
		width: 100%;
		height: 100%;
		display: flex;
	}
	.bin-list {
		flex: 3;
		display: flex;
		flex-direction: column;
		background-color: darkgray;
		padding: 12px;
		gap: 8px;
		overflow-y: auto;
	}
	.selection-viewer {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.canvas-container {
		flex: 7;
		display: flex;
		align-items: center;
	}
	span {
		font-family: 'Figtree';
	}
	canvas {
		aspect-ratio: 4 / 3;
		width: min(100%, 100vh * 4/ 3);
	}
</style>
