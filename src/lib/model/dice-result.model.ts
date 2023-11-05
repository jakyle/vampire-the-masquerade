import { isActionResultBase, type ActionResultBase } from './action-result-base.model';

export interface DiceResult extends ActionResultBase {
	successes: number;
	criticals: number;
	beastialFailure: boolean;
	messyCritical: boolean;
	succeeded: boolean;
	rolls: number[];
	hungerRolls: number[];
}

export const isDiceResult = (result?: ActionResultBase): result is DiceResult => {
	const requiredKeys: Array<keyof DiceResult> = [
		'successes',
		'criticals',
		'beastialFailure',
		'messyCritical',
		'rolls',
		'succeeded',
		'hungerRolls'
	];

	if (!isActionResultBase(result)) {
		return false;
	}

	for (const key of requiredKeys) {
		if (!(key in result)) {
			return false;
		}
	}

	const record = result as DiceResult;

	return (
		typeof record?.successes === 'number' &&
		typeof record?.criticals === 'number' &&
		typeof record?.beastialFailure === 'boolean' &&
		typeof record?.messyCritical === 'boolean' &&
		Array.isArray(record?.rolls) &&
		record?.rolls.every((item) => typeof item === 'number') &&
		Array.isArray(record.hungerRolls) &&
		record.hungerRolls?.every((item) => typeof item === 'number')
	);
};
