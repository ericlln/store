<script lang="ts">
	import Button from '$lib/Generic/Button.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import { Backend } from '$lib/Util/Backend';

	let itemName = '';
	let notes = '';
	let quantity = '';

	let location = '';

	const addItem = async () => {
		await Backend.AddItem(1, itemName, parseInt(quantity), notes);
	};
</script>

<div class="layout-wrapper">
	<Input bind:value={itemName} height={50} placeholder="Item Name" maxlength={64} />
	<div class="d">
		<div class="smaller">
			<Input bind:value={location} placeholder="Store" maxlength={64} />
			<Input bind:value={quantity} placeholder="Quantity" maxlength={64} />
		</div>
		<div class="smaller">
			<textarea bind:value={notes} placeholder={'Notes'} />
		</div>
	</div>
	<div>
		<Button>Cancel</Button>
		<Button on:click={addItem}>Save</Button>
	</div>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.layout-wrapper {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		padding: 14px;
	}
	.d {
		display: flex;
		flex-direction: row;
	}
	.smaller {
		display: flex;
		flex: 1;
		flex-direction: column;
	}
</style>
