<script lang="ts">
	import { goto } from '$app/navigation';
	import type { ActionLog } from '$lib/model/action-log';
	import type { CharacterInfo } from '$lib/model/character-info.model';
	import type { Character, Hunger } from '$lib/model/character.model';
	import { deleteCharacter, updateCharacter } from '$lib/store/characters';
	import { createBuffer } from '$lib/util/buffer';
	import CharacterCard from './CharacterCard.svelte';

	export let characters: Character[];
	export let actionLog: ActionLog | undefined = undefined;

	export let selectedCharacters: CharacterInfo[] = characters
		.filter((character) => character.isActive)
		.map((character) => {
			return {
				...character,
				selected: false
			};
		});

	$: if (characters || actionLog) {
		selectedCharacters = characters
			.filter((character) => character.isActive)
			.map((character) => ({
				...character,
				selected: false,
				actionResult: actionLog?.actionResults.find(
					(actionResult) => actionResult.characterId === character.id
				)
			}));
	}

	const onEdit = (event: CustomEvent<string>) => {
		const id = event.detail;
		goto(`character-sheet/${id}`);
	};

	const selectCharacter = ({ detail: id }: CustomEvent<string>) => {
		const newCharacters = [...selectedCharacters];
		const index = newCharacters.findIndex((character) => character.id === id);
		newCharacters[index].selected = !newCharacters[index].selected;
		selectedCharacters = newCharacters;
	};

	const deleteWithId = (event: CustomEvent<string>) => {
		const id = event.detail;
		deleteCharacter(id);
	};

	const hungerBuffer = createBuffer((payload: Partial<Character>) => updateCharacter(payload), 250);
	const updateHunger = (event: CustomEvent<{ hunger: Hunger; id: string }>) => {
		hungerBuffer(event.detail);
	};
</script>

<div class="flex flex-1 flex-col gap-8 p-8">
	{#if selectedCharacters.length > 0}
		<div class="flex h-3/4 w-full flex-col justify-between gap-2">
			<ul class="flex w-full justify-center gap-8 py-4">
				{#each selectedCharacters as character (character.id)}
					<li id={character.id}>
						<CharacterCard
							{character}
							on:selectCharacter={selectCharacter}
							on:editCharacter={onEdit}
							on:updateHunger={updateHunger}
							on:deleteCharacter={deleteWithId}
						/>
					</li>
				{/each}
			</ul>
		</div>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<div class="flex h-full w-full flex-col items-center justify-center gap-2">
				<h1 class="animate-pulse text-9xl uppercase text-red-800">no characters!!!</h1>
				<p class="text-4xl text-blue-600">
					Click the the plus sign on the top left to add a characters
				</p>
			</div>
		</div>
	{/if}
</div>
