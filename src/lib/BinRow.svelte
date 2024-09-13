<script lang="ts">
	import Button from './Generic/Button.svelte';
	import type { Bin } from './Util/Models';

	export let bin: Bin;
</script>

<Button height="30px" type="space-between" padding="10px">
	<span>{bin.name}</span>
	<span>{`${0} items`}</span>
</Button>
