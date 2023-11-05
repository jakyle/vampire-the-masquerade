<script lang="ts">
	import { charactersStore } from '$lib/store/characters';
	import { addExtensions } from '$lib/util/extension';
	import { passiveRollLogStore } from '$lib/store/roll-logs';
	import { onMount } from 'svelte';
	import Overlay from '$lib/components/Overlay.svelte';
	import Tooltip from '$lib/components/Tooltip.svelte';

	let baseRoute = '/app/';

	const toRoute = (val: string) => `${baseRoute}${val}`;

	addExtensions();

	let isLoaded = false;

	const routes = [
		{
			route: 'main',
			display: '🎲',
			description: 'main app'
		},
		{
			route: 'character-sheet',
			display: '➕',
			description: 'add character'
		}
	];

	onMount(async () => {
		await Promise.all([charactersStore.load(), passiveRollLogStore.load()]);
		isLoaded = true;
	});
	let elements = routes.toRecord(
		({ route }) => route,
		({ description }) => ({ el: null as HTMLElement | null, description })
	);
</script>

<div id="app-body"
	class="relative h-screen w-full flex flex-row bg-cover overflow-auto">
	<nav class="flex-none flex flex-col py-4 w-12 items-center bg-neutral-950 gap-8 [&>a]:text-2xl">
		{#each routes as { route, display }}
			<a bind:this={elements[route].el} href={toRoute(route)}>{display}</a>
		{/each}
	</nav>

	<Overlay />
	{#if isLoaded}
		<slot />
	{/if}
</div>

{#each routes as { route }}
	<Tooltip el={elements[route].el} direction="right">
		{elements[route].description}
	</Tooltip>
{/each}
