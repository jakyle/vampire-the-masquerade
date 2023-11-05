<script lang="ts">
	import { isDiceResult } from '$lib/model/dice-result.model';
	import { isPassiveResult } from '$lib/model/passive.model';
	import type { Character } from '../model/character.model';
	import type { ActionLog } from '../model/action-log';
	import PassiveTable from './PassiveTable.svelte';
	import RollTable from './RollTable.svelte';

	export let logs: ActionLog[] = [];
	export let characters: Character[] = [];

	let filter: string = '';

	$: filteredLogs = logs.filter((log) => {
		const timeStamp = new Date(log.log.createdAt).toLocaleString().toLocaleLowerCase().trim();
		const characterNames = log?.actionResults?.map((data) =>
			characters
				.find((character) => character.id === data.id)
				?.name.toLocaleLowerCase()
				.trim()
		);
		const logId = log.log.id.toLocaleLowerCase().trim();

		const filterLowered = filter.toLocaleLowerCase().trim();

		return (
			timeStamp.includes(filterLowered) ||
			characterNames.some((name) => name?.includes(filterLowered)) ||
			logId.includes(filterLowered)
		);
	});

	$: characterMap = characters.reduce((acc, character) => {
		acc[character.id] = character;
		return acc;
	}, {} as Record<string, Character>);
</script>

<div class="flex h-full flex-col gap-1 bg-inherit py-2 cursor-auto">
	<ul class="flex flex-1 flex-col-reverse gap-2 overflow-auto">
		{#each filteredLogs as log}
			<li class="flex flex-col gap-1 rounded bg-gray-900 px-2 py-4">
				<p class="text-sm text-stone-50">🔑 {log.log.id}</p>
				<h4 class="h-10 text-lg font-bold text-stone-100">
					📅 {new Date(log.log.createdAt).toLocaleString()}
				</h4>
				<div class="flex flex-row flex-wrap justify-center gap-2">
					{#each log.actionResults as result}
						{#if isPassiveResult(result)}
							<PassiveTable
								name={characterMap?.[result.characterId]?.name ?? '<character removed>'}
								passive={result}
							/>
						{:else if isDiceResult(result)}
							<RollTable
								name={characterMap?.[result.characterId]?.name ?? '<character removed>'}
								roll={result}
							/>
						{/if}
					{/each}
				</div>
			</li>
		{/each}
	</ul>
	<div class="flex h-8 flex-none items-center">
		<div class="flex flex-none items-center justify-center bg-stone-100 px-2 py-2">🔎</div>
		<input bind:value={filter} class="w-full flex-1 px-4 py-2" />
	</div>
</div>
