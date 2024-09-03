<script lang="ts">
	import { Util } from '$lib/Util/Util';
	import ButtonPopup from './ButtonPopup.svelte';
	import PopupManager from './PopupManager.svelte';

	let popupManager: PopupManager;

	export let size: string = 'm';
	export let type: string = ''; // s, m, l, xl
	export let disabled: boolean = false;

	export let popupText: string = '';
	export let popupDir: string = 't'; // t, b, l, r
	export let popupPadding: number = 10;

	const onMouseEnter = (evt: MouseEvent) => {
		if (!popupText) return;

		const ele = evt.target as HTMLButtonElement;
		if (!ele) return;

		const rect = ele.getBoundingClientRect();

		popupManager.addPopup(
			'buttonPopup',
			Util.ComponentToDom(ButtonPopup, {
				popupText,
				popupDir,
				popupPadding,
				parentRect: rect
			})
		);
	};

	const onMouseLeave = () => {
		if (!popupText) return;
		popupManager.removePopup('buttonPopup');
	};
</script>

<PopupManager bind:this={popupManager} onRemovePopup={(id) => {}} />
<button
	class={`btn ${size} ${type}`}
	{disabled}
	on:click
	on:mouseenter={onMouseEnter}
	on:mouseleave={onMouseLeave}
>
	<slot></slot>
</button>

<style>
	.btn {
		border: none;
		cursor: pointer;
		pointer-events: auto;
		border-radius: 2px;
		transition:
			background-color 0.3s,
			color 0.3s;
	}
	.btn:hover {
		opacity: 0.8;
	}
	.btn:disabled {
		cursor: not-allowed;
	}

	.btn.s {
		padding: 4px;
		font-size: 12px;
	}

	.btn.m {
		padding: 6px;
		font-size: 14px;
	}

	.btn.l {
		padding: 8px 16px;
		font-size: 16px;
	}

	.btn.xl {
		font-size: 32px;
	}

	.btn.icon {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 10px;
	}
</style>
