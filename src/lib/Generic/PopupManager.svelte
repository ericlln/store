<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	export let createElement: boolean = false; // Needed for event listener
	export let onRemovePopup: (id: string) => void = () => {};

	const popups = new Map<string, HTMLElement>();
	const controller = new AbortController();
	let ref: HTMLSpanElement;

	onMount(() => {
		if (createElement) {
			ref = document.createElement('span');
			ref.className = 'PopupManager';
			document.body.appendChild(ref);
		}
	});

	export function addPopup(id: string, popup: HTMLElement): void {
		if (popups.has(id)) return;

		popups.set(id, popup);
		document.body.appendChild(popup);
	}

	export function removePopup(id: string): void {
		if (!popups.has(id)) return;
		onRemovePopup(id);
		document.body.removeChild(popups.get(id)!);
		popups.delete(id);
	}

	export function hasPopup(id: string): boolean {
		return popups.has(id);
	}

	export function addGlobalListener(eventType: string, callback: EventListener) {
		if (!createElement) return;
		document.addEventListener(eventType, callback, { signal: controller.signal });
	}

	onDestroy(() => {
		for (const popup of popups.values()) {
			document.body.removeChild(popup);
		}
		popups.clear();

		// Remove all event listeners
		controller.abort();
		if (createElement) ref.remove();
	});
</script>
