<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import NavHeader from '$lib/Generic/NavHeader.svelte';
	import PathPicker from '$lib/Generic/PathPicker.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { onMount } from 'svelte';

	let origPath: string | null = null;
	let path: string | null = null;
	let origStoreName = $page.params.slug;
	let storeName = origStoreName;
	let storeEdited = false;

	// let spaceCnt: number | null = null;
	// let binCnt: number | null = null;
	// let itemCnt: number | null = null;

	onMount(async () => {
		origPath = path = await Backend.OpenStore(storeName);
	});

	const isStoreModified = (): boolean => {
		if (storeName !== origStoreName) return true;
		if (path !== origPath) return true;

		return false;
	};

	const handlePathSelected = (e: CustomEvent) => {
		path = e.detail.path;
		storeEdited = isStoreModified();
	};

	const handleNameInput = () => {
		storeEdited = isStoreModified();
	};
</script>

{#if path}
	<div class="layout-wrapper">
		<NavHeader
			title="Store Overview"
			onBack={() => {
				goto(`/stores`);
			}}
		/>

		<div class="inputs-container">
			<Input width="100%" height={50} bind:value={storeName} on:input={handleNameInput} />
			<div class="path-editor">
				<span class="text path">{path}</span>
				<PathPicker on:pathSelected={handlePathSelected} />
			</div>
		</div>

		<div class="actions-container">
			<Button
				fontSize="xl"
				type="space-between"
				padding="0px 12px"
				width="100%"
				on:click={() => {
					goto(`/stores/${storeName}/spaces`);
				}}
			>
				<Icon
					viewBox="0 -960 960 960"
					path="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h240v-560H200v560Zm320 0h240v-280H520v280Zm0-360h240v-200H520v200Z"
					size={64}
				/>
				<span class="text">Spaces</span>
			</Button>

			<Button
				fontSize="xl"
				type="space-between"
				padding="0px 12px"
				width="100%"
				on:click={async () => {}}
			>
				<Icon
					viewBox="0 -960 960 960"
					path="M200-80q-33 0-56.5-23.5T120-160v-451q-18-11-29-28.5T80-680v-120q0-33 23.5-56.5T160-880h640q33 0 56.5 23.5T880-800v120q0 23-11 40.5T840-611v451q0 33-23.5 56.5T760-80H200Zm0-520v440h560v-440H200Zm-40-80h640v-120H160v120Zm200 280h240v-80H360v80Zm120 20Z"
					size={64}
				/>
				<span class="text">Bins</span>
			</Button>

			<Button
				fontSize="xl"
				type="space-between"
				padding="0px 12px"
				width="100%"
				on:click={async () => {}}
			>
				<Icon
					viewBox="0 -960 960 960"
					path="m260-520 220-360 220 360H260ZM700-80q-75 0-127.5-52.5T520-260q0-75 52.5-127.5T700-440q75 0 127.5 52.5T880-260q0 75-52.5 127.5T700-80Zm-580-20v-320h320v320H120Zm580-60q42 0 71-29t29-71q0-42-29-71t-71-29q-42 0-71 29t-29 71q0 42 29 71t71 29Zm-500-20h160v-160H200v160Zm202-420h156l-78-126-78 126Zm78 0ZM360-340Zm340 80Z"
					size={64}
				/>
				<span class="text">Items</span>
			</Button>
		</div>

		<Button fontSize="l" width="100%" padding="10px 0px" disabled={!storeEdited}>Save</Button>
	</div>
{:else}
	<h1>Error</h1>
{/if}

<style>
	* {
		box-sizing: border-box;
	}
	.text {
		font-family: 'Figtree';
	}
	.text.path {
		width: 80%;
		word-wrap: break-word;
		white-space: normal;
		overflow-wrap: break-word;
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
	.path-editor {
		width: 100%;
		display: flex;
		justify-content: space-between;
	}
	.inputs-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}
	.actions-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 10px;
		overflow-y: auto;
	}
</style>
