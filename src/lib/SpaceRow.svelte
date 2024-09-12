<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Generic/Button.svelte';
	import type { Space } from './Util/Models';

	export let storeName: string;
	export let space: Space;

	const handleEdit = () => {
		goto(`/store/${storeName}/spaces/${space.id}`);
	};
</script>

<Button on:click={handleEdit} height="30px" type="space-between">
	<span>{space.name}</span>
	<span>1</span>
</Button>

<style>
	* {
		box-sizing: border-box;
	}
	/* .row-container {
		width: 100%;
		display: flex;
		justify-content: space-between;
		background-color: grey;
		padding: 8px;
		border-radius: 4px;
	} */
</style>
