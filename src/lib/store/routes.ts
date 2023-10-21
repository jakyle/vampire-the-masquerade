import { writable } from "svelte/store";

export type Routes = "home" | "add" | "edit";

export const routesStore = writable<Routes>("home");
