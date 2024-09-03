<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import type { Point } from '$lib/Mapper/Geometry';
	import { Backend } from '$lib/Util/Backend';

	export let name = '';
	export let tags = '';
	export let location: Point;

	const createStore = async () => {
		await Backend.CreateStore(name, location);
	};
</script>

<div class="layout-wrapper">
	<div class="main-content">
		<div class="inputs-list">
			<Input bind:value={name} placeholder="A name for your store" />
			<Input bind:value={tags} placeholder="Associated with each item in this store" />
		</div>

		<div class="item-list">
			<Button
				on:click={() => {
					goto('/item/editor');
				}}>Add Item</Button
			>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>1</Button>
			<Button>2</Button>
			<Button>3</Button>
			<Button>4</Button>
			<Button>5</Button>
			<Button>1</Button>
			<Button>2</Button>
		</div>
	</div>
	<div class="actions-wrapper">
		<Button on:click={() => goto('/setup/placing')}>Cancel</Button>
		<Button on:click={() => createStore()}>Save</Button>
	</div>
</div>

<style>
	.layout-wrapper {
		display: flex;
		flex-direction: column;
		width: 100vw;
		height: 100vh;
	}
	.main-content {
		display: flex;
		flex-direction: column;
		flex: 1;
		padding: 20px;
		gap: 20px;
		overflow: hidden;
	}
	.inputs-list {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}
	.item-list {
		display: flex;
		flex-direction: column;
		flex: 1;
		gap: 2px;
		background-color: lightblue;
		border: 2px gray solid;
		margin: 6px;
		padding: 2px;
		overflow-y: auto;
	}
	.actions-wrapper {
		display: flex;
		justify-content: space-between;
		padding: 8px 12px;
		background: gray;
		box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.1);
	}
</style>
