export interface ActionResultBase {
	id: string,
	characterName: string;
	characterId: string;
	createdAt: string,
}

export const isActionResultBase = (result: unknown): result is ActionResultBase => {
	const requiredKeys: Array<keyof ActionResultBase> = [
		'id',
		'characterName',
		'characterId',
		'createdAt',
	];

	if (typeof result !== 'object' || result === null) {
		return false;
	}

	for (const key of requiredKeys) {
		if (!(key in result)) {
			return false;
		}
	}

	const record = result as Record<string, unknown>;

	return typeof record.id === 'string' &&
		typeof record.characterName === 'string' &&
		typeof record.characterId === 'string' &&
		typeof record.createdAt === 'string';
}