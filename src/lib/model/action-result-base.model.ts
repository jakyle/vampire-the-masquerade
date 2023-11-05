export interface ActionResultBase {
	id: string;
	characterId: string;
	logId: string;
	createdAt: string;
}

export const isActionResultBase = (result: unknown): result is ActionResultBase => {
	const requiredKeys: Array<keyof ActionResultBase> = ['id', 'characterId', 'createdAt', 'logId'];

	if (typeof result !== 'object' || result === undefined || result === null) {
		return false;
	}

	for (const key of requiredKeys) {
		if (!(key in result)) {
			return false;
		}
	}

	const record = result as Record<string, unknown>;

	return (
		typeof record.id === 'string' &&
		typeof record.characterId === 'string' &&
		typeof record.createdAt === 'string' &&
		typeof record.logId === 'string'
	);
};
