// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}

	interface Array<T> {
		tap(callBack: (value: T, index: number, array: T[]) => void): T[];

		toRecord<V, R = Record<V, T>>(keyFn: (value: T, index?: number, array?: T[]) => V): R;
		toRecord<V, U, R = Record<V, U>>(
			keyFn: (value: T, index?: number, array?: T[]) => V,
			valueFn: (value: T, index?: number, array?: T[]) => U
		): R;
		toRecord<V, U, R = Record<V, U>>(
			keyFn: (value: T, index?: number, array?: T[]) => V,
			value: U
		): R;
	}
	interface String {
		toTitleCase(): string;
	}
}

export {};

/// <reference types="svelte" />
/// <reference types="vite/client" />
