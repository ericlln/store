<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Generic/Button.svelte';
	import type { Space } from './Util/Models';

	export let storeName: string;
	export let space: Space;

	const handleEdit = () => {
		goto(`/stores/${storeName}/spaces/${space.id}`);
	};
</script>

<Button on:click={handleEdit} padding="10px" type="space-between">
	<span>{space.name}</span>
	<span>0 Bins</span>
</Button>

<style>
	* {
		box-sizing: border-box;
	}
</style>
