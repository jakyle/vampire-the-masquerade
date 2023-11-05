import type { DiceResult } from './dice-result.model';
import type { PassiveResult } from './passive.model';

export type Log = {
	id: string;
	difficulty: number;
	modifier: number;
	createdAt: string;
};

export type ActionLog = {
	log: Log;
	actionResults: (DiceResult | PassiveResult)[];
};
