<script lang="ts">
	import type { PassiveResult } from '$lib/model/passive.model';
	import Tooltip from './Tooltip.svelte';

	export let passive: PassiveResult;
	export let name: string;

	const keys = Object.keys(passive);

	let elements = keys.toRecord((key) => key, null as HTMLElement | null);
</script>

<table
	class="border-collapse rounded-sm text-2xl text-white
[&>tr>*]:border [&>tr>*]:border-green-950 [&>tr>*]:text-center
[&>tr>td]:bg-green-100 [&>tr>td]:text-stone-900 [&>tr>th]:bg-green-700"
>
	<tr>
		<th class="text-center" colspan={passive.total === undefined ? 2 : 3}>{name}</th>
	</tr>
	<tr>
		{#if passive.total !== undefined}
			<th bind:this={elements['total']}>🧮</th>
		{/if}
		<th bind:this={elements['succeeded']}>🏁</th>
		<th bind:this={elements['hunger']}>🩸</th>
	</tr>
	<tr>
		{#if passive.total !== undefined}
			<td>{passive.total}</td>
		{/if}
		<td>{passive.succeeded ? '✅' : '❌'}</td>
		<td>{passive.hunger}</td>
	</tr>
</table>

{#each keys as key}
	<Tooltip el={elements[key]} direction="top">
		{key}
	</Tooltip>
{/each}
