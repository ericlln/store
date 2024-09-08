<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/Generic/Button.svelte';
	import { Backend } from '$lib/Util/Backend';
	import type { Space } from '$lib/Util/Models';
	import { onMount } from 'svelte';

	const storeName = $page.params.slug;

	let spaces: Space[] | null = null;

	onMount(async () => {
		spaces = await Backend.GetSpaces(storeName);
	});
</script>

<h1>{storeName}</h1>

{#if spaces}
	{#each spaces as space (space.id)}
		<h1>{space.name}</h1>
		<span>{space.id}</span>
	{/each}
{/if}

<Button
	on:click={async () => {
		await goto(`/store/${storeName}/spaces/editor`);
	}}>Create New Space</Button
>
