<script lang="ts">
	import type { CharacterInfo } from '../model/character-info.model';
	import type { ActionInfo } from '$lib/model/action-info.model';
	import { charactersStore } from '../store/characters';
	import { executeAction, passiveRollLogStore } from '../store/roll-logs';
	import Logs from './Logs.svelte';
	import RollForm, { type RollFormValues } from './RollForm.svelte';
	import { rollTension } from '$lib/store/roll-tension';
	import Characters from './Characters.svelte';
	import { debounce } from '$lib/util/debounce';
	import type { ActionLog } from '$lib/model/action-log';
	import bloodLeather from '$lib/assets/blood-leather.jpg';

	let selectedCharacters: CharacterInfo[] = [];
	$: filteredCharacters = selectedCharacters.filter((character) => character.selected);
	$: isCharactersSelected = filteredCharacters.length > 0;

	const sideMenuOptions = ['logs'] as const;

	type SideMenuOption = (typeof sideMenuOptions)[number];

	let sideMenuOption: SideMenuOption | undefined;

	let actionLog: ActionLog | undefined = undefined;

	const action = async (event: CustomEvent<Partial<RollFormValues>>) => {
		const { action, skill, attribute, difficulty, modifier } = event.detail;

		const payload = {
			skill,
			attribute,
			difficulty,
			modifier,
			selectedCharacters: filteredCharacters.map((character) => character.id),
			actionType: action
		};

		actionLog = await executeAction(payload as unknown as ActionInfo);
		rollTension.set('tension');
	};

	let resizing = false;
	let myDiv: HTMLDivElement;
	const openSideMenu = (option: (typeof sideMenuOptions)[number]) => {
		sideMenuOption = option === sideMenuOption ? undefined : option;
	};
	const handleMouseDown = (event: MouseEvent) => {
		if (event.offsetX > myDiv.offsetWidth - 10) {
			resizing = true;
			document.body.classList.add('cursor-e-resize');
		}
	};

	const handleMouseUp = () => {
		resizing = false;
		document.body.classList.remove('cursor-e-resize');
	};

	const handleMouseMove = debounce((event: MouseEvent) => {
		if (resizing) {
			myDiv.style.width = `${event.pageX - myDiv.offsetLeft}px`;
		}
	}, 1.5);
</script>

<svelte:body on:mouseup={handleMouseUp} on:mousemove={handleMouseMove} />

<div 
	style="background-image: url('{bloodLeather}');"
	class="flex h-full w-full flex-row bg-cover bg-blend-overlay bg-neutral-700"
>
	{#await $charactersStore then characters}
		<aside class="flex-none h-full flex-row flex bg-neutral-900">
			<div class="h-full">
				<ul class="flex flex-col w-full py-8 items-center [&>li]:text-3xl [&>li]:cursor-pointer">
					<li>
						<button on:click={() => openSideMenu('logs')}> 🗨️ </button>
					</li>
				</ul>
			</div>

			<div
				bind:this={myDiv}
				role="button"
				data-state={resizing ? 'resize' : undefined}
				tabindex="-1"
				class:hidden={sideMenuOption === undefined}
				on:mousedown={handleMouseDown}
				class="w-72 flex-none resize-x overflow-auto cursor-e-resize transition-colors bg-neutral-800 data-resize:bg-blue-500 data-resize:select-none pr-1 mr-0"
			>
				{#if sideMenuOption === 'logs'}
					<Logs logs={$passiveRollLogStore} {characters} />
				{/if}
			</div>
		</aside>

		<main class="flex-1">
			<Characters bind:selectedCharacters {characters} {actionLog} />
			<RollForm on:rollData={action} disabled={!isCharactersSelected} />
		</main>
	{/await}
</div>
