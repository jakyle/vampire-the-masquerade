<script lang="ts">
	import type { DiceResult } from '$lib/model/dice-result.model';
	import Dice from './Dice.svelte';
	import Tooltip from './Tooltip.svelte';

	export let roll: DiceResult;
	export let name: string;

	let rollEl: HTMLTableElement;

	const keys = Object.keys(roll) as (keyof DiceResult)[];

	let elements = keys.toRecord((key) => key, null as HTMLElement | null);
</script>

<Tooltip el={rollEl} direction="left">
	<div
		class="flex flex-col divide-y-2  p-6 rounded gap-2 [&>ul]:flex [&>ul]:flex-row [&>ul]:gap-4"
	>
		<ul>
			{#each roll.rolls as dice}
				<li>
					<Dice roll={dice} />
				</li>
			{/each}
		</ul>

		<div class="w-full border border-red-950"></div>

		<ul>
			{#each roll.hungerRolls as dice}
				<li>
					<Dice roll={dice} dieType="hunger" />
				</li>
			{/each}
		</ul>
	</div>
</Tooltip>
<table
	bind:this={rollEl}
	class="border-collapse rounded-sm text-2xl text-white [&>tr>*]:border [&>tr>*]:border-green-950 [&>tr>*]:text-center [&>tr>td]:bg-green-100 [&>tr>td]:text-stone-900 [&>tr>th]:bg-green-700"
>
	<tr>
		<th class="text-center" colspan="5">{name}</th>
	</tr>
	<tr>
		<th bind:this={elements.successes}>👍</th>
		<th bind:this={elements.criticals}>💥</th>
		<th bind:this={elements.messyCritical}>🦇</th>
		<th bind:this={elements.bestialFailure}>💩</th>
		<th bind:this={elements.succeeded}>🏁</th>
	</tr>
	<tr>
		<td>{roll.successes}</td>
		<td>{roll.criticals}</td>
		<td>{roll.messyCritical ? '✅' : '❌'}</td>
		<td>{roll.bestialFailure ? '✅' : '❌'}</td>
		<td>{roll.succeeded ? '✅' : '❌'}</td>
	</tr>
</table>

{#each keys as key}
	<Tooltip el={elements[key]} direction="top">
		{key}
	</Tooltip>
{/each}
