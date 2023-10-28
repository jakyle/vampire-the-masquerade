export type DiceResult = {
	characterId: string;
	successes: number;
	criticals: number;
	bestialFailure: boolean;
	messyCritical: boolean;
	succeeded: boolean;
	rolls: number[];
	hungerRolls: number[];
};
