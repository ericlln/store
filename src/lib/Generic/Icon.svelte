<script lang="ts">
	export let viewBox: string;
	export let path: string;
	export let size: number = 24;
	export let type: string = '';
</script>

<svg
	class={`svg ${type}`}
	xmlns="http://www.w3.org/2000/svg"
	{viewBox}
	width={size}
	height={size}
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
>
	<path d={path} />
</svg>

<style>
	.path {
		pointer-events: inherit;
	}
	.svg {
		padding: 6px 2px;
		pointer-events: none;
	}
</style>
