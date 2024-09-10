<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { createEventDispatcher } from 'svelte';
	import Button from './Button.svelte';

	let path: string;
	const dispatch = createEventDispatcher();

	const selectPath = async () => {
		try {
			let selectedPath = await open({
				title: 'Select store location',
				multiple: false,
				directory: true
			});

			if (typeof selectedPath === 'string') {
				path = selectedPath;
				dispatch('pathSelected', { path });
			}
		} catch (err: unknown) {
			console.error(err);
		}
	};
</script>

<Button on:click={selectPath} fontSize="m" padding="8px 20px">Browse</Button>
