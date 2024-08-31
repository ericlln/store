<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/Generic/Navbar.svelte';
</script>

<div>
	<Navbar
		title="Summary"
		handleBack={() => {
			goto('/setup/placing');
		}}
	/>
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
