import { isActionResultBase, type ActionResultBase } from './action-result-base.model';

export interface PassiveResult extends ActionResultBase {
	characterId: string;
	succeeded: boolean;
	hunger: number;
	total: number;
}

export const isPassiveResult = (result?: ActionResultBase): result is PassiveResult => {
	const requiredKeys: Array<keyof PassiveResult> = ['characterId', 'succeeded', 'hunger', 'total'];

	if (typeof result !== 'object' || result === null) {
		return false;
	}

	if (!isActionResultBase(result)) {
		return false;
	}

	for (const key of requiredKeys) {
		if (!(key in result)) {
			return false;
		}
	}

	const record = result as PassiveResult;

	return (
		typeof record?.characterId === 'string' &&
		typeof record?.succeeded === 'boolean' &&
		typeof record?.hunger === 'number' &&
		typeof record?.total === 'number'
	);
};
