<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import NavHeader from '$lib/Generic/NavHeader.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { createGrid } from 'ag-grid-community';
	import type { Item } from '$lib/Util/Models';
	import type { GridOptions, ColDef, GetRowIdParams, GridApi } from 'ag-grid-community';
	import { onMount } from 'svelte';

	import 'ag-grid-community/styles/ag-grid.css';
	import 'ag-grid-community/styles/ag-theme-alpine.css';

	const storeName = $page.params.slug;
	const spaceId = $page.params.spaceId;
	const binId = $page.params.binId;

	let items: Item[] = [];
	let gridApi: GridApi<Item>;
	let gridElement: HTMLDivElement;

	const columnDefs: ColDef<Item>[] = [
		{ field: 'name', filter: 'agTextColumnFilter' },
		{ field: 'spaceName', filter: 'agTextColumnFilter' },
		{ field: 'binName', filter: 'agTextColumnFilter' },
		{ field: 'quantity', filter: 'agNumberColumnFilter' },
		{ field: 'notes' }
	];

	onMount(async () => {
		const resp = await Backend.GetItemList(storeName, null);

		if (resp) {
			items = resp;
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
		title={`All Items`}
		onBack={() => {
			goto(`/stores/${storeName}`);
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
