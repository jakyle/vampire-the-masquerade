export type ActionInfo = {
	skill?: string;
	attribute?: string;
	difficulty: number;
	modifier: number;
	selectedCharacters: string[];
	actionType: 'roll' | 'passive';
};
