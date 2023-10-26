<script lang="ts">
	import type { CharacterInfo } from '../model/character-info.model';
	import type { Character, Hunger } from '../model/character.model';
	import { charactersStore, deleteCharacterById } from '../store/characters';
	import CharacterCard from './CharacterCard.svelte';
	import { passiveRollLogStore } from '../store/roll-logs';
	import { v4 as uuidv4 } from 'uuid';
	import Logs from './Logs.svelte';
	import RollForm, { type RollFormValues } from './RollForm.svelte';
	import { goto } from '$app/navigation';
	import { checkPassives, rollCharacters } from '$lib/util/rolls';

	export let characters: Character[];

	let selectedCharacters: CharacterInfo[] = characters
		.filter((character) => character.isActive)
		.map((character) => {
			return {
				...character,
				selected: false
			};
		});

	$: if (characters) {
		selectedCharacters = characters
			.filter((character) => character.isActive)
			.map((character) => ({
				...character,
				selected: false
			}));
	}

	const onEdit = (event: CustomEvent<string>) => {
		const id = event.detail;
		goto(`character-sheet/${id}`);
	};

	const selectCharacter = (event: CustomEvent<string>) => {
		const id = event.detail;
		const newCharacters = [...selectedCharacters];
		const index = newCharacters.findIndex((character) => character.id === id);
		newCharacters[index].selected = !newCharacters[index].selected;
		selectedCharacters = newCharacters;
	};

	const deleteWithId = (event: CustomEvent<string>) => {
		const id = event.detail;
		deleteCharacterById(id);
	};

	const action = (event: CustomEvent<RollFormValues>) => {
		const { action, skill, attribute, difficulty, modifier } = event.detail;
		let actionType: 'roll' | 'passive';
		let characterPassiveRolls = clear(selectedCharacters);
		switch (action) {
			case 'roll':
				characterPassiveRolls = rollCharacters(
					skill,
					attribute,
					difficulty,
					modifier,
					characterPassiveRolls
				);
				actionType = 'roll';
				break;
			case 'passive':
				characterPassiveRolls = checkPassives(skill, attribute, difficulty, characterPassiveRolls);
				actionType = 'passive';
				break;
		}

		if (
			actionType &&
			characterPassiveRolls &&
			characterPassiveRolls.length > 0 &&
			characterPassiveRolls.some((character) => character[actionType])
		) {
			updateLog(characterPassiveRolls, actionType, difficulty, modifier);
			selectedCharacters = characterPassiveRolls;
		}
	};

	const updateLog = (
		characterInfo: CharacterInfo[],
		key: 'passive' | 'roll',
		difficulty: number,
		modifier: number
	) => {
		passiveRollLogStore.update((log) => {
			const newLog = [...log];
			newLog.push({
				id: uuidv4().toString(),
				difficulty,
				modifier,
				characterData: characterInfo
					.filter((character) => character[key])
					.map((character) => ({
						id: character.id,
						[key]: character[key],
						name: character.name
					})),
				timestamp: Date.now()
			});
			return newLog;
		});
	};

	const clear = (characters: CharacterInfo[]) =>
		characters.map(
			(character) =>
				({
					...character,
					roll: undefined,
					passive: undefined
				} as CharacterInfo)
		);

	const updateHunger = (event: CustomEvent<{ hunger: Hunger; id: string }>) => {
		const { hunger, id } = event.detail;
		const newCharacters = [...$charactersStore];
		const index = newCharacters.findIndex((character) => character.id === id);
		newCharacters[index].hunger = hunger;
		charactersStore.set(newCharacters);
	};
</script>

<div class="flex h-full w-full flex-row">
	<div class="flex h-full w-full flex-1 flex-col gap-8 p-8">
		<div class="w-full">
			<a
				href="/character-sheet"
				class="m-0 flex h-12 w-12 justify-center items-center rounded bg-red-800 p-0 text-3xl">+</a
			>
		</div>
		{#if selectedCharacters.length > 0}
			<div class="flex h-3/4 w-full flex-col justify-between gap-2">
				<ul class="flex w-full justify-center gap-8 py-4">
					{#each selectedCharacters as character}
						<li>
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

				<RollForm on:rollData={action} />
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

	<div class="w-96 flex-none">
		<Logs logs={$passiveRollLogStore} {characters} />
	</div>
</div>
