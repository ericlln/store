<script lang="ts">
	import type { Point } from '$lib/Mapper/Geometry';
	import StoreEditor from '$lib/Store/StoreEditor.svelte';

	export let data: {
		coord: Point;
	};
</script>

<StoreEditor location={data.coord} />
