import type { ActionResultBase } from './action-result-base.model';
import type { Character } from './character.model';

export type CharacterInfo = {
	selected: boolean;
	actionResult?: ActionResultBase;
} & Character;
