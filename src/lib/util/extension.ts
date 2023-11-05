/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/ban-ts-comment */

// @ts-nocheck
function tap<T>(callback: (value: T, index?: number, array?: T[]) => void) {
	this.forEach((element, index, array) => callback(element, index, array));

	return this;
}

function toRecord<T, U>(
	keyFn: (value: T, index?: number, array?: T[]) => string | number | symbol,
	value?: (value: T, index?: number, array?: T[]) => U | U
) {
	return this.reduce(
		(obj, curr, currentIndex: number, array: Array<T>) => ({
			...obj,
			[keyFn(curr, currentIndex, array)]:
				value === undefined
					? curr
					: typeof value === 'function'
					? value(curr, currentIndex, array)
					: value
		}),
		{}
	);
}

function toTitleCase() {
	if (/(?=.*[A-Z]{2,}.*)^[A-Z]*$/.test(this)) {
		return this;
	}
	const result = this.replace(/([A-Z])/g, ' $1');
	const finalResult = result.charAt(0).toUpperCase() + result.substr(1);
	return finalResult;
}

function filterDuplicatedProperties<T extends object>(obj2: T): Partial<T> {
	const result: Partial<T> = {};

	for (const key in this) {
		if (this[key] !== obj2[key]) {
			result[key] = this[key];
		}
	}

	return result;
}

function removeEmptyStringProperties<T extends object>(): Partial<T> {
	const result: Partial<T> = {};

	for (const key in this) {
		if (this[key] !== '') {
			result[key] = this[key];
		}
	}

	return result;
}

export const addExtensions = () => {
	if (!Array.prototype.tap) {
		Array.prototype.tap = tap;
	}

	if (!Array.prototype.toRecord) {
		Array.prototype.toRecord = toRecord;
	}

	if (!String.prototype.toTitleCase) {
		String.prototype.toTitleCase = toTitleCase;
	}

	if (!Object.prototype.filterDuplicatedProperties) {
		Object.prototype.filterDuplicatedProperties = filterDuplicatedProperties;
	}

	if (!Object.prototype.removeEmptyStringProperties) {
		Object.prototype.removeEmptyStringProperties = removeEmptyStringProperties;
	}
};
