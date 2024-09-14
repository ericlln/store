<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import NavHeader from '$lib/Generic/NavHeader.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { createGrid } from 'ag-grid-community';
	import type { Item } from '$lib/Util/Models';
	import type { GridOptions, ColDef, GetRowIdParams, GridApi } from 'ag-grid-community';
	import { onMount } from 'svelte';

	const storeName = $page.params.slug;
	const spaceId = $page.params.spaceId;
	const binId = $page.params.binId;

	let items: Item[] = [];
	let gridApi: GridApi<Item>;
	let gridElement: HTMLDivElement;

	const columnDefs: ColDef<Item>[] = [
		{ field: 'id' },
		{ field: 'binId' },
		{ field: 'spaceId' },
		{ field: 'name' },
		{ field: 'quantity' },
		{ field: 'notes' }
	];

	onMount(async () => {
		const resp = await Backend.GetItemList(storeName, parseInt(binId));

		if (resp) {
			items = resp;
			console.log(items);
		} else {
		}

		const gridOptions: GridOptions<Item> = {
			rowData: items,
			columnDefs,
			getRowId: (params: GetRowIdParams<Item>) => params.data.id.toString()
		};

		gridApi = createGrid(gridElement, gridOptions);
	});

	const createItem = async () => {
		await Backend.CreateItem(storeName, parseInt(spaceId), parseInt(binId), 'test');
	};
</script>

<div class="layout-wrapper">
	<NavHeader
		title={`Items in Bin`}
		onBack={() => {
			goto(`/stores/${storeName}/spaces/${spaceId}/`);
		}}
	/>

	<div class="grid-container">
		<div
			id="datagrid"
			class="ag-theme-alpine"
			style="width: 100%; height: 90%"
			bind:this={gridElement}
		/>
	</div>
</div>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/ag-grid-community@29.0.0/styles/ag-grid.css"
	/>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/ag-grid-community@29.0.0/styles/ag-theme-alpine.css"
	/>
</svelte:head>

<style>
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

	.grid-container {
		width: 100%;
		height: 100%;
	}
</style>
