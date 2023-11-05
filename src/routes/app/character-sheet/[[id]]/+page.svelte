<script lang="ts">
	import Add from '$lib/components/Add.svelte';
	import type { Character } from '$lib/model/character.model';
	import { page } from '$app/stores';
	import { getCharacterById } from '$lib/store/characters';
	import { onDestroy } from 'svelte';

	let character: Character | undefined;
	let isFetched = false;

	const unsubscribe = page.subscribe(async (value) => {
		character = value.params.id ? await getCharacterById(value.params.id) : undefined;
		isFetched = true;
	});

	onDestroy(unsubscribe);
</script>

{#if isFetched}
	<Add {character} />
{/if}
