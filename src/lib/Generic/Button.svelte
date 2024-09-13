<script lang="ts">
	import { Util } from '$lib/Util/Util';
	import ButtonPopup from './ButtonPopup.svelte';
	import PopupManager from './PopupManager.svelte';

	let popupManager: PopupManager;

	export let fontSize: string = 'm'; // s, m, l, xl
	export let type: string = '';
	export let padding: string = '0';
	export let disabled: boolean = false;
	export let width: string = '';
	export let height: string = '';
	export let color: string = '';
	export let bg: string = '#eee';

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

<PopupManager bind:this={popupManager} />
<button
	class={`btn ${fontSize} ${type} `}
	style={`width: ${width}; height: ${height}; padding: ${padding}; color: ${color}; background-color: ${bg}`}
	{disabled}
	on:click
	on:mouseenter={onMouseEnter}
	on:mouseleave={onMouseLeave}
>
	<slot></slot>
</button>

<style>
	.btn {
		font-family: 'Figtree';
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
		font-size: 12px;
	}

	.btn.m {
		font-size: 14px;
	}

	.btn.l {
		font-size: 24px;
	}

	.btn.xl {
		font-size: 32px;
	}

	.btn.centered {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.btn.space-between {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
