<script lang="ts">
	import { page } from '$app/stores';
	import { Backend } from '$lib/Util/Backend';
	import { onMount } from 'svelte';

	let hasStore = false;

	const name = $page.params.slug;
	// let spaceCnt: number | null = null;
	// let binCnt: number | null = null;
	// let itemCnt: number | null = null;

	onMount(async () => {
		hasStore = await Backend.OpenStore(name);
	});
</script>

{#if hasStore}
	<h1>Not error</h1>
{:else}
	<h1>Error</h1>
{/if}
