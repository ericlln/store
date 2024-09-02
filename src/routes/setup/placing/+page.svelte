<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import StorePlacer from '$lib/Mapper/StorePlacer.svelte';
</script>

<div>
	<Navbar
		title="Placing"
		handleBack={() => {
			goto('/setup/shaping');
		}}
		handleNext={() => {
			goto('/setup/summary');
		}}
	/>
	<StorePlacer />
</div>
