// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
	type Primitive = string | number | boolean | symbol | undefined | null | ((...args: unknown[]) => unknown) | Array<unknown>;

	type KeyOfOrValue<T> = T extends Primitive ? T : keyof T | T[keyof T];

	interface Array<T> {
		tap(callBack: (value: T, index: number, array: T[]) => void): T[];

		toRecord<TKey extends KeyOfOrValue<T>, R = Record<TKey, T>>(
			keyFn: (value: T, index?: number, array?: T[]) => TKey
		): R;
		toRecord<TKey extends KeyOfOrValue<T>, UVal, R = Record<TKey, UVal>>(
			keyFn: (value: T, index?: number, array?: T[]) => TKey,
			valueFn: (value: T, index?: number, array?: T[]) => UVal
		): R;
		toRecord<TKey extends KeyOfOrValue<T>, UVal, R = Record<TKey, UVal>>(
			keyFn: (value: T, index?: number, array?: T[]) => TKey,
			value: UVal
		): R;
	}
	interface String {
		toTitleCase(): string;
	}
}

export { };

/// <reference types="svelte" />
/// <reference types="vite/client" />
