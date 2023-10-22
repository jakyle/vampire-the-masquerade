<script lang="ts">
	import Characters from '$lib/components/Characters.svelte';
	import { charactersStore } from '$lib/store/characters';
</script>

<Characters characters={$charactersStore} />
