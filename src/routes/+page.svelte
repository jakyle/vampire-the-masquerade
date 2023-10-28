<script lang="ts">
	import Characters from '$lib/components/Characters.svelte';
	import Overlay from '$lib/components/Overlay.svelte';
	import { charactersStore } from '$lib/store/characters';
</script>


<div class="bg-inherit w-full h-full relative">
	<Overlay />
	<Characters characters={$charactersStore} />
</div>
