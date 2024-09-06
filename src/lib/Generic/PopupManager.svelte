<script lang="ts">
	import { onDestroy } from 'svelte';

	export let onRemovePopup: (id: string) => void = () => {};

	const popups = new Map<string, HTMLElement>();

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

	onDestroy(() => {
		for (const popup of popups.values()) {
			document.body.removeChild(popup);
		}
		popups.clear();
	});
</script>
