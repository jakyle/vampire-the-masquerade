export type AttributeValue = 1 | 2 | 3 | 4 | 5;
export type Hunger = 0 | AttributeValue;
export type SkillValue = Hunger;

export type Character = {
	isActive: boolean;

	id: string;
	name: string;

	strength: AttributeValue;
	dexterity: AttributeValue;
	stamina: AttributeValue;

	charisma: AttributeValue;
	manipulation: AttributeValue;
	composure: AttributeValue;

	intelligence: AttributeValue;
	wits: AttributeValue;
	resolve: AttributeValue;

	athletics: SkillValue;
	brawl: SkillValue;
	crafts: SkillValue;
	drive: SkillValue;
	firearms: SkillValue;
	melee: SkillValue;
	larceny: SkillValue;
	stealth: SkillValue;
	survival: SkillValue;

	animalKen: SkillValue;
	etiquette: SkillValue;
	insight: SkillValue;
	intimidation: SkillValue;
	leadership: SkillValue;
	performance: SkillValue;
	persuasion: SkillValue;
	streetwise: SkillValue;
	subterfuge: SkillValue;

	academics: SkillValue;
	awareness: SkillValue;
	finance: SkillValue;
	investigation: SkillValue;
	medicine: SkillValue;
	occult: SkillValue;
	politics: SkillValue;
	science: SkillValue;
	technology: SkillValue;

	hunger: Hunger;
};
