<script>
	export let color = 'lightgray';
	export let height = '2px';
	export let margin = '0';
</script>

<div
	class="divider"
	style="
      --color: {color}; 
      --height: {height}; 
      --margin: {margin};"
></div>

<style>
	.divider {
		width: 100%;
		background-color: var(--color);
		height: var(--height);
		margin: var(--margin);
	}
</style>
