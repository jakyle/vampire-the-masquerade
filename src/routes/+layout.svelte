<script lang="ts">
	import '../app.css';
	import { innerWidth, innerHeight } from '$lib/store/window';

	let windowInnerWidth: number;
	let windowInnerHeight: number;

	$: $innerWidth = windowInnerWidth;
	$: $innerHeight = windowInnerHeight;
</script>

<svelte:window bind:innerWidth={windowInnerWidth} bind:innerHeight={windowInnerHeight} />

<div id="app-body" class="h-screen w-full bg-stone-900 overflow-auto">
	<slot />
</div>
