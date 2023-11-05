import { invoke } from '@tauri-apps/api';
import { derived, writable } from 'svelte/store';
import type { AddCharacter, Character } from '../model/character.model';
import type { skillNames } from '$lib/data/skills';
import type { attributeNames } from '$lib/data/attributes';
import { createAsyncStore } from './asyncStore';

export const charactersStore = createAsyncStore<Character[]>([], () => {
	return invoke<Array<Character>>('get_characters');
});

export const addCharacter = async (character: AddCharacter) => {
	const postedCharacter = await invoke<Character>('post_character', { character });
	charactersStore.update((characters) => [...characters, postedCharacter]);
};

export const getCharacterById = async (id: string) => {
	const character = await invoke<Character>('get_character_by_id', { id });
	return character;
};

export const deleteCharacter = async (id: string) => {
	await invoke('delete_character', { id });
	charactersStore.update((characters) => characters.filter((character) => character.id !== id));
};

export const characterById = (id: string) =>
	derived(charactersStore, ($characters) => $characters.find((character) => character.id === id));

export const updateCharacter = async (character: Partial<Character>) => {
	const updatedCharacter = await invoke<Character>('patch_character', { character });
	charactersStore.update((characters) =>
		characters.map((character) =>
			character.id === updatedCharacter.id ? updatedCharacter : character
		)
	);
};

type AttributeName = (typeof attributeNames)[number];
export const selectedAttributeStore = writable<AttributeName | ''>('');

type SkillName = (typeof skillNames)[number];
export const selectedSkillStore = writable<SkillName | ''>('');
