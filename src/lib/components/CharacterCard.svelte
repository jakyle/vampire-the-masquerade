<script lang="ts">
	import type { CharacterInfo } from '../model/character-info.model';
	import { createEventDispatcher } from 'svelte';
	import type { Hunger } from '../model/character.model';
	import { selectedAttributeStore, selectedSkillStore } from '$lib/store/characters';
	import DottedRange from './DottedRange.svelte';
	import { isDiceResult } from '$lib/model/dice-result.model';
	import { isPassiveResult } from '$lib/model/passive.model';

	export let character: CharacterInfo;
	let isDeleting: boolean;

	$: selectedSkill = $selectedSkillStore as keyof CharacterInfo | '';
	$: selectedAttribute = $selectedAttributeStore as keyof CharacterInfo | '';

	const dispatchSelectCharacter = createEventDispatcher<{
		selectCharacter: string;
	}>();

	const dispatchEditCharacter = createEventDispatcher<{
		editCharacter: string;
	}>();

	const dispatchUpdateHunger = createEventDispatcher<{
		updateHunger: { hunger: Hunger; id: string };
	}>();

	const dispatchDeleteCharacter = createEventDispatcher<{
		deleteCharacter: string;
	}>();

	const selectWithId = (id: string) => dispatchSelectCharacter('selectCharacter', id);

	const editWithId = (id: string) => dispatchEditCharacter('editCharacter', id);

	const deleteWithId = (id: string) => dispatchDeleteCharacter('deleteCharacter', id);

	const updateHunger = (event: CustomEvent<number>) => {
		dispatchUpdateHunger('updateHunger', {
			hunger: event.detail as Hunger,
			id: character.id
		});
	};
</script>

<div
	class="flex h-96 w-72 flex-col rounded bg-stone-800 outline-blue-400"
	class:outline-offset-2={character.selected}
	class:outline={character.selected}
>
	{#if !isDeleting}
		<div class="flex flex-1 flex-col">
			<div class="flex flex-row gap-1">
				<button
					class:bg-green-800={character.selected}
					class:active:bg-green-950={character.selected}
					class:active:bg-stone-950={!character.selected}
					class:bg-stone-600={!character.selected}
					on:click={() => selectWithId(character.id)}
					class="h-12 flex-1 rounded border-none p-0"
				>
					<h1 class="select-none text-2xl text-white">{character.name}</h1>
				</button>
				<button
					class="h-12 flex-none rounded bg-blue-800 p-2 text-2xl text-white active:bg-blue-950"
					on:click={() => editWithId(character.id)}
				>
					📝
				</button>

				<button
					class="h-12 flex-none rounded bg-yellow-800 p-2 text-2xl text-white active:bg-yellow-950"
					on:click={() => (isDeleting = true)}
				>
					🗑️
				</button>
			</div>

			{#if isDiceResult(character.actionResult)}
				<table
					class="ml-2 w-11/12 font-semibold text-white [&>tr]:border-b [&>tr]:border-b-stone-500"
				>
					<tr>
						<td>successes</td>
						<td>{character.actionResult.successes}</td>
					</tr>

					<tr>
						<td>criticals</td>
						<td>{character.actionResult.criticals}</td>
					</tr>

					<tr>
						<td>messy</td>
						<td>{character.actionResult.messyCritical ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>bestial</td>
						<td>{character.actionResult.beastialFailure ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>succeeded</td>
						<td>{character.actionResult.succeeded ? '✅' : '❌'}</td>
					</tr>
				</table>
			{/if}

			{#if isPassiveResult(character.actionResult)}
				<table
					class="ml-2 w-11/12 font-semibold text-white [&>tr]:border-b [&>tr]:border-b-stone-500"
				>
					<tr>
						<td>succeeded</td>
						<td>{character.actionResult.succeeded ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>hunger</td>
						<td>{character.actionResult.hunger}</td>
					</tr>
				</table>
			{/if}
		</div>
		<table class="h-14 text-white [&>tr>*]:w-1/2 [&>tr>*]:border [&>tr>*]:border-stone-500">
			<tr>
				<th>{selectedAttribute}</th>
				<th>{selectedSkill}</th>
			</tr>
			<tr>
				<td>
					{#if selectedAttribute !== ''}
						{character[selectedAttribute]}
					{/if}
				</td>
				<td>
					{#if selectedSkill !== ''}
						{character[selectedSkill]}
					{/if}
				</td>
			</tr>
		</table>
		<div class="flex-none">
			<DottedRange on:change={updateHunger} bind:value={character.hunger} label="Hunger" />
		</div>
	{:else}
		<div class="flex flex-col justify-center items-center gap-4">
			<div class="flex flex-col gap-6 px-8 py-4">
				<div class="flex flex-col">
					<h1 class="text-2xl text-white">
						Are you sure you want to delete? <span class="uppercase animate-pulse text-yellow-500"
							>this action cannot be undone</span
						>
					</h1>
				</div>
				<div class="w-full flex flex-col gap-y-6">
					<button
						class="flex-none rounded bg-yellow-800 p-2 text-2xl text-white active:bg-yellow-950"
						on:click={() => deleteWithId(character.id)}
					>
						YES, DELETE THIS CHARACTER!
					</button>
					<button
						class="flex-none rounded bg-blue-800 p-2 text-2xl text-white active:bg-blue-950"
						on:click={() => (isDeleting = false)}
					>
						NO, KEEP THIS CHARACTER
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
