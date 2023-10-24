<script lang="ts">
	import type { CharacterInfo } from '../model/character-info.model';
	import { createEventDispatcher } from 'svelte';
	import Slider from './Slider.svelte';
	import type { Hunger } from '../model/character.model';
	import { selectedAttributeStore, selectedSkillStore } from '$lib/store/characters';
	import DottedRange from './DottedRange.svelte';

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

	const updateHunger = (event: unknown) => {
		const eventX = event as Event & {
			target: EventTarget & { value: Hunger | string };
		};
		dispatchUpdateHunger('updateHunger', {
			hunger: +eventX.target.value as Hunger,
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

			{#if character.roll}
				<table
					class="ml-2 w-11/12 font-semibold text-white [&>tr]:border-b [&>tr]:border-b-stone-500"
				>
					<tr>
						<td>successes</td>
						<td>{character.roll.successes}</td>
					</tr>

					<tr>
						<td>criticals</td>
						<td>{character.roll.criticals}</td>
					</tr>

					<tr>
						<td>messy</td>
						<td>{character.roll.messyCritical ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>bestial</td>
						<td>{character.roll.bestialFailure ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>succeeded</td>
						<td>{character.roll.succeeded ? '✅' : '❌'}</td>
					</tr>
				</table>
			{/if}

			{#if character.passive}
				<table
					class="ml-2 w-11/12 font-semibold text-white [&>tr]:border-b [&>tr]:border-b-stone-500"
				>
					<tr>
						<td>succeeded</td>
						<td>{character.passive.succeeded ? '✅' : '❌'}</td>
					</tr>

					<tr>
						<td>hunger</td>
						<td>{character.passive.hunger}</td>
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
			<Slider on:input={updateHunger} bind:value={character.hunger} label="Hunger" />
			<DottedRange />
		</div>
	{:else}
		<div class="flex flex-col justify-center items-center gap-4">
			<div class="flex flex-col gap-2">
				<div class="flex flex-col">
					<h1 class="text-2xl text-white">Are you sure you want to delete character?</h1>
				</div>
				<div class="w-full flex justify-around">
					<button
						class="h-12 flex-none w-20 rounded bg-green-800 p-2 text-2xl text-white active:bg-green-950"
						on:click={() => deleteWithId(character.id)}
					>
						✅
					</button>
					<button
						class="h-12 flex-none w-20 rounded bg-red-800 p-2 text-2xl text-white active:bg-red-950"
						on:click={() => (isDeleting = false)}
					>
						❌
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
