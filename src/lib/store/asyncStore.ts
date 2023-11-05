import { writable, type Writable } from 'svelte/store';

type AsyncStore<T> = Writable<T> & { load: () => Promise<void> };

export function createAsyncStore<T>(initialValue: T, fetchData: () => Promise<T>): AsyncStore<T> {
	const { subscribe, set, update } = writable<T>(initialValue);

	return {
		subscribe,
		set,
		update,
		load: async () => {
			const data = await fetchData();
			set(data);
		}
	};
}
