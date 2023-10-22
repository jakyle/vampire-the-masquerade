import type { CharacterInfo } from '$lib/model/character-info.model';
import type { Character } from '$lib/model/character.model';
import type { DiceResult } from '$lib/model/dice-result.model';
import type { PassiveResult } from '$lib/model/passive.model';

export const rollCharacters = (
	selectedSkill: string,
	selectedAttribute: string,
	difficulty: number,
	modifier: number,
	selectedCharacters: CharacterInfo[]
) => selectedCharacters.map((character) => {
	if (character.selected) {
		const skill = selectedSkill ? (character[selectedSkill as keyof Character] as number) : 0;
		const attribute = selectedAttribute
			? (character[selectedAttribute as keyof Character] as number)
			: 0;

		const dicePoolWithoutHunger = Math.max(skill + attribute + modifier, 0);

		const hunger = character.hunger;

		const rolls = Array(dicePoolWithoutHunger - Math.min(dicePoolWithoutHunger, hunger))
			.fill(0)
			.map(() => Math.floor(Math.random() * 10) + 1);

		const hungerRolls = Array(Math.min(dicePoolWithoutHunger, hunger))
			.fill(0)
			.map(() => Math.floor(Math.random() * 10) + 1);

		const successes =
			rolls.filter((roll) => roll >= 6).length + hungerRolls.filter((roll) => roll >= 6).length;

		const halfMessyCritical = hungerRolls.filter((roll) => roll === 10).length;

		const halfCritical = rolls.filter((roll) => roll === 10).length;

		const criticals = Math.floor(halfMessyCritical + halfCritical / 2);

		const messyCritical = criticals > 0 && halfMessyCritical > 0;

		const succeeded = successes >= difficulty;

		const bestialFailure = hungerRolls.filter((roll) => roll === 1).length >= 1 && !succeeded;

		const roll: DiceResult = {
			successes: successes + criticals,
			criticals,
			messyCritical,
			bestialFailure,
			succeeded
		};

		return {
			...character,
			roll
		};
	} else {
		return { ...character };
	}
});

export const checkPassives = (
	selectedSkill: string,
	selectedAttribute: string,
	difficulty: number,
	selectedCharacters: CharacterInfo[]
) => selectedCharacters.map((character) => {
	if (character.selected) {
		const skill = selectedSkill ? (character[selectedSkill as keyof CharacterInfo] as number) : 0;
		const attribute = selectedAttribute
			? (character[selectedAttribute as keyof CharacterInfo] as number)
			: 0;
		const hunger = character.hunger;

		const passive: PassiveResult = {
			hunger,
			succeeded: skill + attribute >= difficulty
		};

		return {
			...character,
			passive
		};
	} else {
		return { ...character };
	}
});
