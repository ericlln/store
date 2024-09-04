<script lang="ts">
	import { goto } from '$app/navigation';
	import { appWindow, PhysicalSize } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	onMount(async () => {
		try {
			await appWindow.setMinSize(new PhysicalSize(640, 360));
			goto('/dashboard');
		} catch (err: unknown) {
			console.error(`Failed to initialize: ${err}`);
		}
	});
</script>
