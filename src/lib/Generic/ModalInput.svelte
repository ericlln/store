<!-- Fullscreen input for one field -->

<script lang="ts">
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';
	import Input from './Input.svelte';
	import PopupManager from './PopupManager.svelte';

	export let id: string;
	export let parent: PopupManager;
	let value: string;

	const onCancel = () => {
		parent.removePopup(id);
		history.back();
	};

	const onComplete = () => {
		const event = new CustomEvent('spaceNamed', { detail: { value } });
		document.dispatchEvent(event);
		parent.removePopup(id);
	};
</script>

<div class="modal-background">
	<div class="modal-content">
		<div class="modal-inputs">
			<Input bind:value placeholder="Name of space" height={50} width={500} />
			<Button on:click={onComplete}>
				<Icon
					viewBox="0 -960 960 960"
					path="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"
					size={30}
				/>
			</Button>
		</div>

		<Button on:click={onCancel}
			><Icon
				viewBox="0 -960 960 960"
				path="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
				size={18}
			/></Button
		>
	</div>
</div>

<style>
	.modal-background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
		border: none;
		padding: 0;
	}

	.modal-content {
		display: flex;
		flex-direction: column;
		background-color: white;
		padding: 20px;
		gap: 8px;
		border-radius: 8px;
		max-width: 90%;
		max-height: 90%;
	}

	.modal-inputs {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 4px;
	}
</style>
