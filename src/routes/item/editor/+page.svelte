<script lang="ts">
	import { goto } from '$app/navigation';
	import Navbar from '$lib/Generic/Navbar.svelte';
	import ItemEditor from '$lib/Item/ItemEditor.svelte';
</script>

<div>
	<Navbar
		title="New Item"
		handleBack={() => {
			goto('/setup/placing');
		}}
		handleNext={() => {}}
	/>
	<ItemEditor />
</div>

<style>
	* {
		box-sizing: border-box;
	}
	:global(html, body) {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
</style>
