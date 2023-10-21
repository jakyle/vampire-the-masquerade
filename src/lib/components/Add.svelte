<script lang="ts">
  import { onMount } from "svelte";
  import type {
    Character,
    AttributeValue,
    SkillValue,
  } from "../model/character.model";
  import { charactersStore, editCharacterStore } from "../store/characters";
  import { routesStore } from "../store/routes";
  import Slider from "./Slider.svelte";
  import { v4 as uuidv4 } from "uuid";

  onMount(() => {
    id = $editCharacterStore;
    character = $charactersStore.find((character) => character.id === id);

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

  let id: string | undefined;
  let character: Character | undefined;

  let name = "";

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

  const upsertCharacter = () => {
    const upsertCharacter: Character = {
      id: id ?? uuidv4().toString(),
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
      hunger: character?.hunger ?? 1,
    };

    charactersStore.update((characters) => {
      const index = characters.findIndex((character) => character.id === id);

      if (index === -1) {
        characters.push(upsertCharacter);
      } else {
        characters[index] = upsertCharacter;
      }

      return characters;
    });
    routesStore.set("home");
  };

  const cancel = () => {
    routesStore.set("home");
  };
</script>

<form
  class="flex h-full w-full flex-col gap-2 px-20 py-8"
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

    <button
      on:click={cancel}
      type="button"
      class="rounded bg-red-800 px-4 py-2 uppercase text-white">cancel</button
    >
  </div>

  <h1 class="text-center text-xl uppercase text-white">attributes</h1>

  <div class="grid grid-flow-col grid-rows-3 gap-6">
    <Slider min={1} bind:value={strength} label="strength" />
    <Slider min={1} bind:value={dexterity} label="dexterity" />
    <Slider min={1} bind:value={stamina} label="stamina" />

    <Slider min={1} bind:value={charisma} label="charisma" />
    <Slider min={1} bind:value={manipulation} label="manipulation" />
    <Slider min={1} bind:value={composure} label="composure" />

    <Slider min={1} bind:value={intelligence} label="intelligence" />
    <Slider min={1} bind:value={wits} label="wits" />
    <Slider min={1} bind:value={resolve} label="resolve" />
  </div>

  <h1 class="text-center text-lg uppercase text-white">skills</h1>

  <div class="grid grid-flow-col grid-rows-9 gap-x-6 gap-y-3">
    <Slider bind:value={athletics} label="athletics" />
    <Slider bind:value={brawl} label="brawl" />
    <Slider bind:value={crafts} label="crafts" />
    <Slider bind:value={drive} label="drive" />
    <Slider bind:value={firearms} label="firearms" />
    <Slider bind:value={melee} label="melee" />
    <Slider bind:value={larceny} label="larceny" />
    <Slider bind:value={stealth} label="stealth" />
    <Slider bind:value={survival} label="survival" />

    <Slider bind:value={animalKen} label="animal ken" />
    <Slider bind:value={etiquette} label="etiquette" />
    <Slider bind:value={insight} label="insight" />
    <Slider bind:value={intimidation} label="intimidation" />
    <Slider bind:value={leadership} label="leadership" />
    <Slider bind:value={performance} label="performance" />
    <Slider bind:value={persuasion} label="persuasion" />
    <Slider bind:value={streetwise} label="streetwise" />
    <Slider bind:value={subterfuge} label="subterfuge" />

    <Slider bind:value={academics} label="academics" />
    <Slider bind:value={awareness} label="awareness" />
    <Slider bind:value={finance} label="finance" />
    <Slider bind:value={investigation} label="investigation" />
    <Slider bind:value={medicine} label="medicine" />
    <Slider bind:value={occult} label="occult" />
    <Slider bind:value={politics} label="politics" />
    <Slider bind:value={science} label="science" />
    <Slider bind:value={technology} label="technology" />
  </div>

  <div class="w-full py-2">
    <button
      class="w-full rounded bg-green-800 px-4 py-2 uppercase text-white"
      type="submit">save character</button
    >
  </div>
</form>
