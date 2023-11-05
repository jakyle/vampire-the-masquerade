type Callback<T> = (value: T) => void;

export function createBuffer<T>(callback: Callback<T>, delay: number) {
	let timeoutId: NodeJS.Timeout | null = null;
	let lastValue: T | null = null;

	return (value: T) => {
		lastValue = value;

		if (timeoutId !== null) {
			clearTimeout(timeoutId);
		}

		timeoutId = setTimeout(() => {
			if (lastValue !== null) {
				callback(lastValue);
				lastValue = null;
			}
		}, delay);
	};
}
