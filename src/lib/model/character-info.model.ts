import type { Character } from './character.model';
import type { DiceResult } from './dice-result.model';
import type { PassiveResult } from './passive.model';

export type CharacterInfo = {
	selected: boolean;
	roll?: DiceResult;
	passive?: PassiveResult;
} & Character;
