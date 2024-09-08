<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { onMount } from 'svelte';

	let path: string | null = null;

	const name = $page.params.slug;
	// let spaceCnt: number | null = null;
	// let binCnt: number | null = null;
	// let itemCnt: number | null = null;

	onMount(async () => {
		path = await Backend.OpenStore(name);
	});
</script>

{#if path}
	<h1>{name}</h1>
	<h1>{path}</h1>
	<Button
		on:click={async () => {
			await goto(`/store/${name}/spaces`);
		}}>Spaces</Button
	>
	<Button>Items</Button>
	<Button></Button>
{:else}
	<h1>Error</h1>
{/if}
