<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Generic/Button.svelte';
	import Icon from '$lib/Generic/Icon.svelte';
	import { appWindow, PhysicalSize } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	const iconSize = 48;

	onMount(async () => {
		try {
			await appWindow.setMinSize(new PhysicalSize(640, 360));
		} catch (err: unknown) {
			console.error(`Failed to initialize: ${err}`);
		}
	});
</script>

<div class="layout-wrapper">
	<Button
		fontSize="xl"
		type="space-between"
		width="100%"
		padding="20px"
		on:click={() => {
			goto('/stores');
		}}
	>
		<Icon
			viewBox="0 -960 960 960"
			path="M754-81q-8 0-15-2.5T726-92L522-296q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l85-85q6-6 13-8.5t15-2.5q8 0 15 2.5t13 8.5l204 204q6 6 8.5 13t2.5 15q0 8-2.5 15t-8.5 13l-85 85q-6 6-13 8.5T754-81Zm0-95 29-29-147-147-29 29 147 147ZM205-80q-8 0-15.5-3T176-92l-84-84q-6-6-9-13.5T80-205q0-8 3-15t9-13l212-212h85l34-34-165-165h-57L80-765l113-113 121 121v57l165 165 116-116-43-43 56-56H495l-28-28 142-142 28 28v113l56-56 142 142q17 17 26 38.5t9 45.5q0 24-9 46t-26 39l-85-85-56 56-42-42-207 207v84L233-92q-6 6-13 9t-15 3Zm0-96 170-170v-29h-29L176-205l29 29Zm0 0-29-29 15 14 14 15Zm549 0 29-29-29 29Z"
			size={iconSize}
		/>
		<span>All Stores</span></Button
	>

	<Button fontSize="xl" type="space-between" width="100%" padding="20px">
		<Icon
			viewBox="0 -960 960 960"
			path="M300-360h60v-160h-60v50h-60v60h60v50Zm100-50h320v-60H400v60Zm200-110h60v-50h60v-60h-60v-50h-60v160Zm-360-50h320v-60H240v60Zm80 450v-80H160q-33 0-56.5-23.5T80-280v-480q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v480q0 33-23.5 56.5T800-200H640v80H320ZM160-280h640v-480H160v480Zm0 0v-480 480Z"
			size={iconSize}
		/>
		<span>Settings</span></Button
	>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.layout-wrapper {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 100px;
	}
	span {
		font-family: 'Figtree';
	}
</style>
