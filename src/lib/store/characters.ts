import { writable } from "svelte/store";
import type { Character } from "../model/character.model";

export const charactersStore = writable<Character[]>(
  JSON.parse(
    localStorage.getItem("characters") ?? "[]",
  ) as unknown as Character[],
);

charactersStore.subscribe((value) => {
  localStorage.setItem("characters", JSON.stringify(value));
});

export const editCharacterStore = writable<string | undefined>(undefined);
