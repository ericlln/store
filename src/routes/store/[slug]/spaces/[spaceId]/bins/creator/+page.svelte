<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import Input from '$lib/Generic/Input.svelte';
	import NavHeader from '$lib/Generic/NavHeader.svelte';
	import { Backend } from '$lib/Util/Backend';

	const storeName = $page.params.slug;
	const spaceId = $page.params.spaceId;
	$: x = $page.url.searchParams.get('x');
	$: y = $page.url.searchParams.get('y');

	let name = '';

	const onSave = async () => {
		if (!x || !y) return;

		await Backend.CreateBin(storeName, parseInt(spaceId), name, parseFloat(x), parseFloat(y));
	};
</script>

<div class="layout-wrapper">
	<NavHeader
		title="New Bin"
		onBack={() => {
			goto(`/store/${storeName}/spaces/${spaceId}`);
		}}
	/>
	<Input width="100%" height={50} bind:value={name} placeholder="Bin name" />

	<Button fontSize="l" width="100%" padding="10px 0px" on:click={onSave}>Save</Button>
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
</style>
