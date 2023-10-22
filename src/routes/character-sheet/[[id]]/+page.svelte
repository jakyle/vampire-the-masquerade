<script lang="ts">
	import type { PageData } from './$types';
	import Add from '$lib/components/Add.svelte';

	export let data: PageData;
</script>

<Add character={data.character} />
