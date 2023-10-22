import type { Character } from '$lib/model/character.model';
import type { PageLoad } from './$types';

export const load: PageLoad<{ character?: Character }> = async ({ params }) => {
	const characters = JSON.parse(window.localStorage.getItem('characters') ?? '[]') as Character[];
	return { character: characters.find((character: Character) => character.id === params.id) };
};
