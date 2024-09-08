<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import PopupManager from './PopupManager.svelte';

	export let id: string;
	export let parent: PopupManager;
	export let title: string = '';
	export let description: string = '';

	const dispatch = createEventDispatcher();

	const onClose = () => {
		parent.removePopup(id);
	};
</script>

<button class="modal-background" on:click={onClose} aria-label="Close modal">
	<div class="modal-content">
		<h2>{title}</h2>
		<p>{@html description}</p>
	</div>
</button>

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
		background-color: white;
		padding: 20px;
		border-radius: 8px;
		max-width: 80%;
		max-height: 80%;
		overflow-y: auto;
		text-align: center;
	}

	h2 {
		margin-top: 0;
	}

	p {
		margin-top: 10px;
		word-wrap: break-word;
		line-height: 1.6;
	}
</style>
