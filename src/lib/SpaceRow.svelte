<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Generic/Button.svelte';
	import Icon from './Generic/Icon.svelte';
	import type { Space } from './Util/Models';

	export let storeName: string;
	export let space: Space;

	export let spaceName: string = '';
	export let spaceId: number = 0;

	const handleEdit = () => {
		goto(`/store/${storeName}/spaces/editor/${space.id}`);
	};
</script>

<div class="row-container">
	<span>{space.name}</span>
	<Button on:click={handleEdit}>
		<Icon
			viewBox="0 -960 960 960"
			path="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"
			size={24}
		/></Button
	>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.row-container {
		width: 100%;
		display: flex;
		justify-content: space-between;
		background-color: aquamarine;
		padding: 8px;
		border-radius: 10px;
	}
</style>
