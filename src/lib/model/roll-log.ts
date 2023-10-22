import type { DiceResult } from './dice-result.model';
import type { PassiveResult } from './passive.model';

export type CharacterRollPassive = {
	id: string;
	roll?: DiceResult;
	passive?: PassiveResult;
};

export type RollPassiveLog = {
	id: string;
	characterData: CharacterRollPassive[];
	timestamp: number;
};
