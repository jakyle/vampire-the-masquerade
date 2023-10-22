import { writable } from 'svelte/store';
import type { RollPassiveLog } from '../model/roll-log';

export const passiveRollLogStore = writable<RollPassiveLog[]>(
	JSON.parse(localStorage.getItem('roll-log') ?? '[]') as unknown as RollPassiveLog[]
);

passiveRollLogStore.subscribe((value) => {
	localStorage.setItem('roll-log', JSON.stringify(value));
});
