<script lang="ts">
  import { onMount } from "svelte";
  import type { CharacterInfo } from "../model/character-info.model";
  import type { Character, Hunger } from "../model/character.model";
  import type { DiceResult } from "../model/dice-result.model";
  import type { PassiveResult } from "../model/passive.model";
  import { charactersStore, editCharacterStore } from "../store/characters";
  import { routesStore } from "../store/routes";
  import CharacterCard from "./CharacterCard.svelte";
  import { passiveRollLogStore } from "../store/roll-logs";
  import { v4 as uuidv4 } from "uuid";
  import Logs from "./Logs.svelte";
  import RollForm, { type RollFormValues } from "./RollForm.svelte";

  export let characters: Character[];

  let selectedCharacters: CharacterInfo[] = characters
    .filter((character) => character.isActive)
    .map((character) => {
      return {
        ...character,
        selected: false,
      };
    });

  onMount(() => {
    editCharacterStore.set(undefined);
  });

  const onAdd = () => {
    routesStore.set("add");
  };

  const onEdit = (event: CustomEvent<string>) => {
    const id = event.detail;
    editCharacterStore.set(id);
    routesStore.set("add");
  };

  const selectCharacter = (event: CustomEvent<string>) => {
    const id = event.detail;
    const newCharacters = [...selectedCharacters];
    const index = newCharacters.findIndex((character) => character.id === id);
    newCharacters[index].selected = !newCharacters[index].selected;
    selectedCharacters = newCharacters;
  };

  const action = (event: CustomEvent<RollFormValues>) => {
    const { action, skill, attribute, difficulty, modifier } = event.detail;
    switch (action) {
      case "roll":
        roll(skill, attribute, difficulty, modifier);
        break;
      case "passive":
        passive(skill, attribute, difficulty);
        break;
      default:
        break;
    }
  };

  const roll = (selectedSkill: string, selectedAttribute: string, difficulty: number, modifier: number) => {
    clear();
    const selectedCharactersWithRoll = selectedCharacters.map((character) => {
      if (character.selected) {
        const skill = selectedSkill ? (character[selectedSkill as keyof Character] as number) : 0;
        const attribute = selectedAttribute
          ? (character[selectedAttribute as keyof Character] as number)
          : 0;

        const dicePoolWithoutHunger = Math.max(skill + attribute + modifier, 0);

        const hunger = character.hunger;

        const rolls = Array(
          dicePoolWithoutHunger - Math.min(dicePoolWithoutHunger, hunger),
        )
          .fill(0)
          .map(() => Math.floor(Math.random() * 10) + 1);

        const hungerRolls = Array(Math.min(dicePoolWithoutHunger, hunger))
          .fill(0)
          .map(() => Math.floor(Math.random() * 10) + 1);

        const successes =
          rolls.filter((roll) => roll >= 6).length +
          hungerRolls.filter((roll) => roll >= 6).length;

        const halfMessyCritical = hungerRolls.filter(
          (roll) => roll === 10,
        ).length;

        const halfCritical = rolls.filter((roll) => roll === 10).length;

        const criticals = Math.floor(halfMessyCritical + halfCritical / 2);

        const messyCritical = criticals > 0 && halfMessyCritical > 0;

        const succeeded = successes >= difficulty;

        const bestialFailure =
          hungerRolls.filter((roll) => roll === 1).length >= 1 && !succeeded;

        const roll: DiceResult = {
          successes: successes + criticals,
          criticals,
          messyCritical,
          bestialFailure,
          succeeded,
        };

        return {
          ...character,
          roll,
        };
      } else {
        return { ...character };
      }
    });

    passiveRollLogStore.update((log) => {
      const newLog = [...log];
      newLog.push({
        id: uuidv4().toString(),
        characterData: selectedCharactersWithRoll
          .filter((character) => character.roll)
          .map((character) => ({
            id: character.id,
            roll: character.roll,
          })),
        timestamp: Date.now(),
      });
      return newLog;
    });

    selectedCharacters = selectedCharactersWithRoll;
  };

  const passive = (selectedSkill: string, selectedAttribute: string, difficulty: number) => {
    clear();
    const selectedCharactersWithPassive = selectedCharacters.map(
      (character) => {
        if (character.selected) {
          const skill = selectedSkill
            ? (character[selectedSkill as keyof CharacterInfo] as number)
            : 0;
          const attribute = selectedAttribute
            ? (character[selectedAttribute as keyof CharacterInfo] as number)
            : 0;
          const hunger = character.hunger;

          const passive: PassiveResult = {
            hunger,
            succeeded: skill + attribute >= difficulty,
          };

          return {
            ...character,
            passive,
          };
        } else {
          return { ...character };
        }
      },
    );

    console.log(selectedCharactersWithPassive);

    passiveRollLogStore.update((log) => {
      const newLog = [...log];
      newLog.push({
        id: uuidv4().toString(),
        characterData: selectedCharactersWithPassive
          .filter((character) => character.passive)
          .map((character) => ({
            id: character.id,
            passive: character.passive,
          })),
        timestamp: Date.now(),
      });
      return newLog;
    });

    selectedCharacters = selectedCharactersWithPassive;
  };

  const clear = () => {
    selectedCharacters = selectedCharacters.map((character) => {
      return {
        ...character,
        roll: undefined,
        passive: undefined,
      };
    });
  };

  const updateHunger = (event: CustomEvent<{ hunger: Hunger; id: string }>) => {
    const { hunger, id } = event.detail;
    const newCharacters = [...$charactersStore];
    const index = newCharacters.findIndex((character) => character.id === id);
    newCharacters[index].hunger = hunger;
    charactersStore.set(newCharacters);
  };
</script>

<div class="flex h-full w-full flex-row">
  <div class="flex h-full w-full flex-1 flex-col gap-8 p-8">
    <div class="w-full">
      <button
        on:click={onAdd}
        class="m-0 flex h-12 w-12 items-center justify-center rounded bg-red-800 p-0 text-3xl"
        >+</button
      >
    </div>
    {#if selectedCharacters.length > 0}
      <div class="flex h-3/4 w-full flex-col justify-between gap-2">
        <ul class="flex w-full justify-center gap-8 py-4">
          {#each selectedCharacters as character}
            <li>
              <CharacterCard
                {character}
                {selectedAttribute}
                {selectedSkill}
                on:selectCharacter={selectCharacter}
                on:editCharacter={onEdit}
                on:updateHunger={updateHunger}
              />
            </li>
          {/each}
        </ul>

        <RollForm on:rollData={action}></RollForm>

      </div>
    {:else}
      <div class="flex h-full w-full items-center justify-center">
        <div
          class="flex h-full w-full flex-col items-center justify-center gap-2"
        >
          <h1 class="animate-pulse text-9xl uppercase text-red-800">
            no characters!!!
          </h1>
          <p class="text-4xl text-blue-600">
            Click the the plus sign on the top left to add a characters
          </p>
        </div>
      </div>
    {/if}
  </div>

  <div class="w-96 flex-none">
    <Logs logs={$passiveRollLogStore} {characters} />
  </div>
</div>
