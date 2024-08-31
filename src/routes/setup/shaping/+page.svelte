<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import MapShaper from '$lib/Mapper/MapShaper.svelte';

	let mapShaper: MapShaper;
</script>

<div>
	<Navbar
		title="Create Space"
		handleBack={() => {
			goto('/');
		}}
		handleNext={() => {
			mapShaper.sendDrawing();
			goto('/setup/placing');
		}}
	/>
	<MapShaper bind:this={mapShaper} />
</div>

<style>
	:global(html, body) {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
</style>
