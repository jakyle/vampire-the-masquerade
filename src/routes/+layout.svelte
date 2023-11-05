<script lang="ts">
	import '../app.css';
	import { innerWidth, innerHeight } from '$lib/store/window';
	import { addExtensions } from '$lib/util/extension';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { onDestroy } from 'svelte';

	addExtensions();

	let windowInnerWidth: number;
	let windowInnerHeight: number;

	$: $innerWidth = windowInnerWidth;
	$: $innerHeight = windowInnerHeight;

	const unsubscribe = page.subscribe((p) => {
		if (p.url.pathname === '/') {
			goto('/app/main');
		}
	});

	onDestroy(unsubscribe);
</script>

<svelte:window bind:innerWidth={windowInnerWidth} bind:innerHeight={windowInnerHeight} />

<slot />
