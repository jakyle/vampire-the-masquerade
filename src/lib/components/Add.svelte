<script lang="ts">
	import { onMount } from 'svelte';
	import type {
		Character,
		AttributeValue,
		SkillValue,
		AddCharacter
	} from '../model/character.model';
	import { addCharacter, updateCharacter } from '../store/characters';
	import DottedRange from './DottedRange.svelte';
	import { v4 as uuidv4 } from 'uuid';
	import { goto } from '$app/navigation';
	import bloodLeather from '$lib/assets/blood-leather.jpg';

	export let character: Character | undefined;

	onMount(() => {
		if (character) {
			name = character.name;

			strength = character.strength;
			dexterity = character.dexterity;
			stamina = character.stamina;

			charisma = character.charisma;
			manipulation = character.manipulation;
			composure = character.composure;

			intelligence = character.intelligence;
			wits = character.wits;
			resolve = character.resolve;

			athletics = character.athletics;
			brawl = character.brawl;
			crafts = character.crafts;
			drive = character.drive;
			firearms = character.firearms;
			melee = character.melee;
			larceny = character.larceny;
			stealth = character.stealth;
			survival = character.survival;

			animalKen = character.animalKen;
			etiquette = character.etiquette;
			insight = character.insight;
			intimidation = character.intimidation;
			leadership = character.leadership;
			performance = character.performance;
			persuasion = character.persuasion;
			streetwise = character.streetwise;
			subterfuge = character.subterfuge;

			academics = character.academics;
			awareness = character.awareness;
			finance = character.finance;
			investigation = character.investigation;
			medicine = character.medicine;
			occult = character.occult;
			politics = character.politics;
			science = character.science;
			technology = character.technology;
		}
	});

	let name = '';

	let strength: AttributeValue = 1;
	let dexterity: AttributeValue = 1;
	let stamina: AttributeValue = 1;

	let charisma: AttributeValue = 1;
	let manipulation: AttributeValue = 1;
	let composure: AttributeValue = 1;

	let intelligence: AttributeValue = 1;
	let wits: AttributeValue = 1;
	let resolve: AttributeValue = 1;

	let athletics: SkillValue = 0;
	let brawl: SkillValue = 0;
	let crafts: SkillValue = 0;
	let drive: SkillValue = 0;
	let firearms: SkillValue = 0;
	let melee: SkillValue = 0;
	let larceny: SkillValue = 0;
	let stealth: SkillValue = 0;
	let survival: SkillValue = 0;

	let animalKen: SkillValue = 0;
	let etiquette: SkillValue = 0;
	let insight: SkillValue = 0;
	let intimidation: SkillValue = 0;
	let leadership: SkillValue = 0;
	let performance: SkillValue = 0;
	let persuasion: SkillValue = 0;
	let streetwise: SkillValue = 0;
	let subterfuge: SkillValue = 0;

	let academics: SkillValue = 0;
	let awareness: SkillValue = 0;
	let finance: SkillValue = 0;
	let investigation: SkillValue = 0;
	let medicine: SkillValue = 0;
	let occult: SkillValue = 0;
	let politics: SkillValue = 0;
	let science: SkillValue = 0;
	let technology: SkillValue = 0;

	const upsertCharacter = async () => {
		const upsertCharacter: AddCharacter = {
			id: character?.id ?? uuidv4().toString(),
			name,

			strength,
			dexterity,
			stamina,

			charisma,
			manipulation,
			composure,

			intelligence,
			wits,
			resolve,

			athletics,
			brawl,
			crafts,
			drive,
			firearms,
			melee,
			larceny,
			stealth,
			survival,

			animalKen,
			etiquette,
			insight,
			intimidation,
			leadership,
			performance,
			persuasion,
			streetwise,
			subterfuge,
			academics,

			awareness,
			finance,
			investigation,
			medicine,
			occult,
			politics,
			science,
			technology,
			isActive: true,
			hunger: character?.hunger ?? 1
		};

		if (character) {
			const upsert = upsertCharacter.filterDuplicatedProperties(character);
			upsert.id = character.id;
			await updateCharacter(upsert);
		} else {
			await addCharacter(upsertCharacter);
		}

		goto('/app/main');
	};
</script>

<div class="flex w-full h-full justify-center items-center bg-cover bg-blend-luminosity bg-stone-800" style="background-image: url('{bloodLeather}');">
	<form
		class="flex flex-col gap-2 px-20 py-8 overflow-auto bg-stone-700/70 rounded-lg"
		on:submit|preventDefault={upsertCharacter}
	>
		<div class="flex h-16 w-full flex-row justify-between">
			<div>
				<label
					for="name"
					class="mb-2 block text-sm font-medium uppercase text-stone-900 dark:text-white"
					>name</label
				>
				<input
					id="name"
					autocomplete="off"
					type="text"
					bind:value={name}
					class="w-full px-4 py-2 text-lg"
				/>
			</div>

			<a
				href="/app/main"
				type="button"
				class="grid items-center rounded bg-red-800 px-4 py-2 uppercase text-white">cancel</a
			>
		</div>

		<h1 class="text-center text-xl uppercase text-white">attributes</h1>

		<div class="grid grid-flow-col grid-rows-3 gap-y-3 gap-x-12">
			<DottedRange min={1} bind:value={strength} label="strength" />
			<DottedRange min={1} bind:value={dexterity} label="dexterity" />
			<DottedRange min={1} bind:value={stamina} label="stamina" />

			<DottedRange min={1} bind:value={charisma} label="charisma" />
			<DottedRange min={1} bind:value={manipulation} label="manipulation" />
			<DottedRange min={1} bind:value={composure} label="composure" />

			<DottedRange min={1} bind:value={intelligence} label="intelligence" />
			<DottedRange min={1} bind:value={wits} label="wits" />
			<DottedRange min={1} bind:value={resolve} label="resolve" />
		</div>

		<h1 class="text-center text-lg uppercase text-white">skills</h1>

		<div class="grid grid-flow-col grid-rows-9 gap-y-3 gap-x-12">
			<DottedRange bind:value={athletics} label="athletics" />
			<DottedRange bind:value={brawl} label="brawl" />
			<DottedRange bind:value={crafts} label="crafts" />
			<DottedRange bind:value={drive} label="drive" />
			<DottedRange bind:value={firearms} label="firearms" />
			<DottedRange bind:value={melee} label="melee" />
			<DottedRange bind:value={larceny} label="larceny" />
			<DottedRange bind:value={stealth} label="stealth" />
			<DottedRange bind:value={survival} label="survival" />

			<DottedRange bind:value={animalKen} label="animal ken" />
			<DottedRange bind:value={etiquette} label="etiquette" />
			<DottedRange bind:value={insight} label="insight" />
			<DottedRange bind:value={intimidation} label="intimidation" />
			<DottedRange bind:value={leadership} label="leadership" />
			<DottedRange bind:value={performance} label="performance" />
			<DottedRange bind:value={persuasion} label="persuasion" />
			<DottedRange bind:value={streetwise} label="streetwise" />
			<DottedRange bind:value={subterfuge} label="subterfuge" />

			<DottedRange bind:value={academics} label="academics" />
			<DottedRange bind:value={awareness} label="awareness" />
			<DottedRange bind:value={finance} label="finance" />
			<DottedRange bind:value={investigation} label="investigation" />
			<DottedRange bind:value={medicine} label="medicine" />
			<DottedRange bind:value={occult} label="occult" />
			<DottedRange bind:value={politics} label="politics" />
			<DottedRange bind:value={science} label="science" />
			<DottedRange bind:value={technology} label="technology" />
		</div>

		<div class="w-full py-2">
			<button class="w-full rounded bg-green-800 px-4 py-2 uppercase text-white" type="submit"
				>save character</button
			>
		</div>
	</form>
</div>
