<script lang="ts">
	import { page } from '$app/stores';
	import type { Store } from '$lib/Mapper/Store';
	import StoreEditor from '$lib/Store/StoreEditor.svelte';
	import { Backend } from '$lib/Util/Backend';
	import { onMount } from 'svelte';

	const id = parseInt($page.params.slug);
	let store: Store | null = null;

	onMount(async () => {
		store = await Backend.FetchStore(id);
	});
</script>

{#if store}
	<StoreEditor name={store.name} location={store?.location} />
{:else}
	<h1>Error</h1>
{/if}
