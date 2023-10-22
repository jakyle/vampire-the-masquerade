import { derived, writable } from 'svelte/store';
import type { Character } from '../model/character.model';
import type { skillNames } from '$lib/data/skills';
import type { attributeNames } from '$lib/data/attributes';

export const charactersStore = writable<Character[]>(
	JSON.parse(localStorage.getItem('characters') ?? '[]') as unknown as Character[]
);

export const characterById = (id: string) =>
	derived(charactersStore, ($characters) => $characters.find((character) => character.id === id));

charactersStore.subscribe((value) => {
	localStorage.setItem('characters', JSON.stringify(value));
});

type AttributeName = (typeof attributeNames)[number];
export const selectedAttributeStore = writable<AttributeName | ''>('');

type SkillName = (typeof skillNames)[number];
export const selectedSkillStore = writable<SkillName | ''>('');
