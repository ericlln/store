<script lang="ts">
	import { page } from '$app/stores';
	import { Canvas } from '$lib/Mapper/Canvas';
	import { Backend } from '$lib/Util/Backend';
	import type { Space } from '$lib/Util/Models';
	import { onMount } from 'svelte';

	const storeName = $page.params.slug;
	const id = $page.params.id;
	let space: Space;

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;

	onMount(async () => {
		ctx = canvas.getContext('2d');
		const resp = await Backend.GetSpace(storeName, parseInt(id));

		if (resp && ctx) {
			space = resp;
			Canvas.Resize(canvas);
			if (space.drawing) Canvas.DrawPaths(ctx, space.drawing);
		}
	});
</script>

{#if space}
	<h1>{space.name}</h1>
{/if}
<canvas bind:this={canvas} />
