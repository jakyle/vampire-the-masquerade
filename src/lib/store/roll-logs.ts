import { writable } from 'svelte/store';
import type { RollPassiveLog } from '../model/roll-log';

export const passiveRollLogStore = writable<RollPassiveLog[]>(
	(JSON.parse(localStorage.getItem('roll-log') ?? '[]') as RollPassiveLog[])
		.map(roll => ({
			...roll,
			characterData: roll.characterData.filter(character => character.name !== undefined)
		}))
		.filter(roll => (roll.characterData.length > 0))
);

passiveRollLogStore.subscribe((value) => {
	localStorage.setItem('roll-log', JSON.stringify(value));
});
