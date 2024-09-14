<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Generic/Button.svelte';
	import type { Bin } from './Util/Models';

	export let storeName: string;
	export let bin: Bin;
</script>

<Button
	height="30px"
	type="space-between"
	padding="10px"
	on:click={() => {
		goto(`/stores/${storeName}/spaces/${bin.spaceId}/bins/${bin.id}`);
	}}
>
	<span>{bin.name}</span>
	<span>{`${0} items`}</span>
</Button>
