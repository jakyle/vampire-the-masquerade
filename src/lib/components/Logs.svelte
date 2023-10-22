<script lang="ts">
	import type { Character } from '../model/character.model';
	import type { RollPassiveLog } from '../model/roll-log';
	export let logs: RollPassiveLog[];
	export let characters: Character[];

	let filter: string = '';

	$: filteredLogs = logs.filter((log) => {
		const timeStamp = new Date(log.timestamp).toLocaleString().toLocaleLowerCase().trim();
		const characterNames = log.characterData.map((data) =>
			characters
				.find((character) => character.id === data.id)
				?.name.toLocaleLowerCase()
				.trim()
		);
		const logId = log.id.toLocaleLowerCase().trim();

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

	$: console.log(logs);
</script>

<div class="flex h-full flex-col gap-1 bg-stone-400 px-2 py-2">
	<ul class="flex flex-1 flex-col-reverse gap-2 overflow-auto">
		{#each filteredLogs.reverse() as log}
			<li class="flex flex-col gap-1 rounded bg-sky-800 px-2 py-4">
				<p class="text-sm text-stone-50">🔑 {log.id}</p>
				<h4 class="h-10 text-lg font-bold text-stone-100">
					📅 {new Date(log.timestamp).toLocaleString()}
				</h4>
				<div class="flex flex-row flex-wrap justify-center gap-2">
					{#each log.characterData as data}
						{#if data.passive !== undefined}
							<table
								class="border-collapse rounded-sm text-2xl text-white [&>tr>*]:border [&>tr>*]:border-green-950 [&>tr>*]:text-center [&>tr>td]:bg-green-100 [&>tr>td]:text-stone-900 [&>tr>th]:bg-green-700"
							>
								<tr>
									<th class="text-center" colspan="2">{characterMap[data.id].name}</th>
								</tr>
								<tr>
									<th>🏁</th>
									<th>🩸</th>
								</tr>
								<tr>
									<td>{data.passive.succeeded ? '✅' : '❌'}</td>
									<td>{data.passive.hunger}</td>
								</tr>
							</table>
						{/if}

						{#if data.roll !== undefined}
							<table
								class="border-collapse rounded-sm text-2xl text-white [&>tr>*]:border [&>tr>*]:border-green-950 [&>tr>*]:text-center [&>tr>td]:bg-green-100 [&>tr>td]:text-stone-900 [&>tr>th]:bg-green-700"
							>
								<tr>
									<th class="text-center" colspan="5">{characterMap[data.id].name}</th>
								</tr>
								<tr>
									<th>👍</th>
									<th>💥</th>
									<th>💩</th>
									<th>🦇</th>
									<th>🏁</th>
								</tr>
								<tr>
									<td>{data.roll.successes}</td>
									<td>{data.roll.criticals}</td>
									<td>{data.roll.messyCritical ? '✅' : '❌'}</td>
									<td>{data.roll.bestialFailure ? '✅' : '❌'}</td>
									<td>{data.roll.succeeded ? '✅' : '❌'}</td>
								</tr>
							</table>
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
