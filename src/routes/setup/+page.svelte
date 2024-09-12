<script lang="ts">
	import { goto } from '$app/navigation';
	import Divider from '$lib/Generic/Divider.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import PathPicker from '$lib/Generic/PathPicker.svelte';
	import StoreRow from '$lib/Store/StoreRow.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { onMount } from 'svelte';

	let name = '';
	let path = '';

	let storeList: string[];

	onMount(async () => {
		const list = await Backend.GetStoreList();
		if (list) {
			storeList = list;
		}
	});

	const handleSave = async () => {
		if (name === '' || path === '') {
			//todo toast
			return;
		}

		const resp = await Backend.CreateStore(name, path);
		if (!resp) return; //todo toast

		goto('/stores');
	};

	const handlePathSelected = (e: CustomEvent) => {
		path = e.detail.path;
	};
</script>

<div class="page-container">
	<Navbar
		title="Create store"
		nextLabel="Save"
		handleBack={() => {
			goto('/');
		}}
		handleNext={handleSave}
	/>
	<div class="main-wrapper">
		<div class="store-list">
			{#if storeList}
				{#each storeList as store}
					<StoreRow name={store} path="" />
				{/each}
			{/if}
		</div>
		<div class="input-list">
			<div class="input">
				<span class="text">Store name</span>
				<Input bind:value={name} placeholder="Name" width="250px" height={34} />
			</div>
			<Divider />
			<div class="input">
				<span class="text">Location</span>
				<PathPicker on:pathSelected={handlePathSelected} />
			</div>
			<div class="input">
				<span class="text">Selected Path</span>
				<span class="text path">{path || 'None'}</span>
			</div>
		</div>
	</div>
</div>

<style>
	.page-container {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
	}
	.main-wrapper {
		flex: 1;
		display: flex;
		overflow: hidden;
	}
	.store-list {
		flex: 3;
		display: flex;
		flex-direction: column;
		background-color: darkgray;
		padding: 12px;
		overflow-y: auto;
	}
	.input-list {
		display: flex;
		flex-direction: column;
		flex: 7;
		align-items: center;
		background-color: gray;
		padding: 16px;
		gap: 12px;
		overflow: hidden;
	}
	.input {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.text.path {
		text-decoration: underline;
		font-size: medium;
		pointer-events: all;
		cursor: not-allowed;
		color: aqua;
	}
	.text {
		font-family: 'Figtree';
		font-size: large;
		user-select: none;
		pointer-events: none;
	}
</style>
