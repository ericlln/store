<script lang="ts">
	import { onMount, tick } from 'svelte';

	export let maxWidth: string = '200px';
	export let popupText: string = '';
	export let popupDir: string = 't'; // t, b, l, r
	export let popupPadding: number = 10;
	export let parentRect: DOMRect;
	let ref: HTMLDivElement;

	onMount(async () => {
		await tick(); // Wait for DOM to finish loading before getting client rect
		const rect = ref.getBoundingClientRect();

		switch (popupDir) {
			case 't':
				ref.style.left = `${parentRect.x + (parentRect.width - rect.width) / 2}px`;
				ref.style.top = `${parentRect.y - rect.height - popupPadding}px`;
				break;
			case 'b':
				ref.style.left = `${parentRect.x + (parentRect.width - rect.width) / 2}px`;
				ref.style.top = `${parentRect.y + parentRect.height + popupPadding}px`;
				break;
			case 'l':
				ref.style.left = `${parentRect.x - rect.width - popupPadding}px`;
				ref.style.top = `${parentRect.y + (parentRect.height - rect.height) / 2}px`;
				break;
			case 'r':
				ref.style.left = `${parentRect.x + parentRect.width + popupPadding}px`;
				ref.style.top = `${parentRect.y + (parentRect.height - rect.height) / 2}px`;
				break;
		}
	});
</script>

<div bind:this={ref} class="popup" style={`max-width: ${maxWidth}`}>
	{popupText}
</div>

<style>
	.popup {
		position: absolute;
		font-family: 'Figtree';
		background-color: #333;
		color: white;
		padding: 5px;
		border-radius: 3px;
		white-space: normal;
		word-wrap: break-word;
		overflow-wrap: break-word;
		font-size: 12px;
		z-index: 500;
		pointer-events: none;
		overflow: hidden;
	}
</style>
