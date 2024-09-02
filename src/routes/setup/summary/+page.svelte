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
