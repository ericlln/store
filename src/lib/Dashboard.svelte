<script lang="ts">
	import { onMount } from 'svelte';
	import Button from './Generic/Button.svelte';
	import Icon from './Generic/Icon.svelte';
	import { goto } from '$app/navigation';

	onMount(async () => {});
</script>

<div class="layout-wrapper">
	<div class="row">
		<div class="column">
			<Button
				size="xl"
				type="icon"
				on:click={() => {
					goto('/setup/shaping');
				}}
			>
				<Icon
					viewBox="0 -960 960 960"
					path="M754-81q-8 0-15-2.5T726-92L522-296q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l85-85q6-6 13-8.5t15-2.5q8 0 15 2.5t13 8.5l204 204q6 6 8.5 13t2.5 15q0 8-2.5 15t-8.5 13l-85 85q-6 6-13 8.5T754-81Zm0-95 29-29-147-147-29 29 147 147ZM205-80q-8 0-15.5-3T176-92l-84-84q-6-6-9-13.5T80-205q0-8 3-15t9-13l212-212h85l34-34-165-165h-57L80-765l113-113 121 121v57l165 165 116-116-43-43 56-56H495l-28-28 142-142 28 28v113l56-56 142 142q17 17 26 38.5t9 45.5q0 24-9 46t-26 39l-85-85-56 56-42-42-207 207v84L233-92q-6 6-13 9t-15 3Zm0-96 170-170v-29h-29L176-205l29 29Zm0 0-29-29 15 14 14 15Zm549 0 29-29-29 29Z"
					size={64}
				/>
				<span>Setup</span></Button
			>
		</div>
		<div class="column">
			<Button size="xl" type="icon" on:click={() => goto('/store/1')}>
				<Icon
					viewBox="0 -960 960 960"
					path="M320-280q17 0 28.5-11.5T360-320q0-17-11.5-28.5T320-360q-17 0-28.5 11.5T280-320q0 17 11.5 28.5T320-280Zm0-160q17 0 28.5-11.5T360-480q0-17-11.5-28.5T320-520q-17 0-28.5 11.5T280-480q0 17 11.5 28.5T320-440Zm0-160q17 0 28.5-11.5T360-640q0-17-11.5-28.5T320-680q-17 0-28.5 11.5T280-640q0 17 11.5 28.5T320-600Zm120 320h240v-80H440v80Zm0-160h240v-80H440v80Zm0-160h240v-80H440v80ZM200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h560v-560H200v560Zm0-560v560-560Z"
					size={64}
				/>
				<span>All Items</span></Button
			>
		</div>
	</div>
	<div class="row">
		<div class="column">
			<Button size="xl" type="icon" disabled={true}>
				<Icon
					viewBox="0 -960 960 960"
					path="M300-360h60v-160h-60v50h-60v60h60v50Zm100-50h320v-60H400v60Zm200-110h60v-50h60v-60h-60v-50h-60v160Zm-360-50h320v-60H240v60Zm80 450v-80H160q-33 0-56.5-23.5T80-280v-480q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v480q0 33-23.5 56.5T800-200H640v80H320ZM160-280h640v-480H160v480Zm0 0v-480 480Z"
					size={64}
				/>
				<span>About</span></Button
			>
		</div>
		<div class="column">
			<Button size="xl" type="icon" disabled={true}>
				<Icon
					viewBox="0 -960 960 960"
					path="M300-360h60v-160h-60v50h-60v60h60v50Zm100-50h320v-60H400v60Zm200-110h60v-50h60v-60h-60v-50h-60v160Zm-360-50h320v-60H240v60Zm80 450v-80H160q-33 0-56.5-23.5T80-280v-480q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v480q0 33-23.5 56.5T800-200H640v80H320ZM160-280h640v-480H160v480Zm0 0v-480 480Z"
					size={64}
				/>
				<span>Settings</span></Button
			>
		</div>
	</div>
</div>

<style>
	* {
		box-sizing: border-box;
	}
	.layout-wrapper {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
		align-items: center;
		height: 100%;
		padding: 100px;
	}
	.row {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		width: 100%;
		gap: 1em;
	}
	.column {
		display: flex;
		flex-direction: column;
		flex-basis: 100%;
		flex: 1;
	}
	span {
		font-family: 'Figtree';
	}
</style>
