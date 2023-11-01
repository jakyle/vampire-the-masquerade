import type { DiceResult } from './dice-result.model';
import type { PassiveResult } from './passive.model';

export type RollPassiveLog = {
	id: string;
	createdAt: string;
	difficulty: number;
	modifier: number;

	results: (DiceResult | PassiveResult)[];
};
