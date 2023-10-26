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
};
