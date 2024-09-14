<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';
	import Divider from '$lib/Generic/Divider.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import PathPicker from '$lib/Generic/PathPicker.svelte';
	import StoreRow from '$lib/StoreRow.svelte';
	import { Backend } from '$lib/Util/Backend';
	import type { Store } from '$lib/Util/Models';
	import { onMount } from 'svelte';

	let name = '';
	let path = '';

	let stores: Store[] = [];

	onMount(async () => {
		await fetch();
	});

	const fetch = async () => {
		const list = await Backend.GetStoreList();
		if (list) {
			stores = list;
		}
	};

	const handleSave = async () => {
		if (name === '' || path === '') {
			//todo toast
			return;
		}

		const resp = await Backend.CreateStore(name, path);
		if (!resp) return; //todo toast

		await fetch();
	};

	const handlePathSelected = (e: CustomEvent) => {
		path = e.detail.path;
	};
</script>

<div class="page-container">
	<Navbar
		title="All Stores"
		nextLabel="Save"
		handleBack={() => {
			goto('/');
		}}
		handleNext={handleSave}
	/>
	<div class="main-wrapper">
		<div class="store-list">
			<Button
				padding="10px"
				type="space-between"
				on:click={async () => {
					await goto(`/`);
				}}
			>
				<span> Import </span>
				<Icon
					viewBox="0 -960 960 960"
					path="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z"
					size={24}
				/>
			</Button>
			<Divider />

			{#each stores as store}
				<StoreRow {store} />
			{/each}
		</div>
		<div class="input-list">
			<div class="input">
				<span class="text">Store name</span>
				<Input bind:value={name} placeholder="Name" width="250px" height={34} />
			</div>
			<Divider color="lightslategray" />
			<div class="input">
				<span class="text">Location</span>
				<PathPicker on:pathSelected={handlePathSelected} />
			</div>
			<div class="input">
				<span class="text">Selected Path</span>
				<span class="text path">{path || 'None'}</span>
			</div>
			<Divider color="lightslategray" />
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
		background-color: lightslategray;
		padding: 12px;
		overflow-y: auto;
		gap: 8px;
	}
	.input-list {
		display: flex;
		flex-direction: column;
		flex: 7;
		align-items: center;
		background-color: ghostwhite;
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
		color: rebeccapurple;
	}
	.text {
		font-family: 'Figtree';
		font-size: large;
		user-select: none;
		pointer-events: none;
	}
</style>
