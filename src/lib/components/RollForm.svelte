<script context="module" lang="ts">
	const schema = zod.object({
		skill: zod.enum([...skillNames, '']).optional(),
		attribute: zod.enum([...attributeNames, '']).optional(),
		modifier: zod.number().min(-50).max(50).default(0),
		difficulty: zod.number().min(0).max(50),
		action: zod.enum(['roll', 'passive'])
	});

	export type RollFormValues = zod.infer<typeof schema>;
</script>

<script lang="ts">
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-zod';
	import * as zod from 'zod';
	import { createEventDispatcher } from 'svelte';
	import { skillNames } from '../data/skills';
	import { attributeNames } from '../data/attributes';
	import { selectedAttributeStore, selectedSkillStore } from '$lib/store/characters';
	import { onDestroy } from 'svelte';
	import Tooltip from './Tooltip.svelte';
	export let disabled = true;
	let buttonEl: null | HTMLButtonElement = null;

	const dispatchRoll = createEventDispatcher<{
		rollData: Partial<RollFormValues>;
	}>();

	const { form, data, setFields } = createForm<RollFormValues>({
		onSubmit: (values) => dispatchRoll('rollData', values.removeEmptyStringProperties()),
		initialValues: {
			skill: '',
			attribute: '',
			modifier: 0,
			difficulty: 0,
			action: 'roll'
		},
		extend: validator({ schema })
	});

	const unsubscribe = data.subscribe((values) => {
		selectedAttributeStore.set(values.attribute ?? '');
		selectedSkillStore.set(values.skill ?? '');
	});

	function inputChangeHandler(
		field: 'difficulty' | 'modifier',
		e: Event & { currentTarget: EventTarget & HTMLInputElement },
		min: number,
		max: number
	) {
		const numericVal = +e.currentTarget.value;

		const value = +$data[field];
		let inputVal = isNaN(numericVal) ? value : Math.min(Math.max(numericVal, min), max);
		setFields(field, inputVal);
	}

	onDestroy(unsubscribe);
</script>

<form use:form class="flex justify-center">
	<div class="flex w-fit flex-col items-center gap-2 rounded bg-stone-700 px-12 py-8">
		<div class="flex flex-col gap-6">
			<div class="flex flex-row justify-between">
				<div class="flex flex-row gap-2">
					<label class="text-lg text-white" for="attribute">Attribute:</label>
					<select id="attribute" name="attribute">
						<option value="" />
						{#each attributeNames as attribute}
							<option value={attribute}>{attribute}</option>
						{/each}
					</select>
				</div>

				<div class="flex flex-row gap-2">
					<label class="text-lg text-white" for="skill">Skill:</label>
					<select id="skill" name="skill">
						<option value="" />
						{#each skillNames as skill}
							<option value={skill}>{skill}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="flex flex-row gap-2">
				<div class="flex flex-row gap-2">
					<label class="text-lg text-white" for="skill">Action:</label>
					<select id="action" name="action">
						<option value="roll">roll</option>
						<option value="passive">passive</option>
					</select>
				</div>

				<div class="flex flex-row gap-2">
					<label class="text-lg text-white" for="modifier">Modifier:</label>
					<input
						on:input={(e) => inputChangeHandler('modifier', e, -50, 50)}
						name="modifier"
						type="number"
						min={-50}
						max="50"
						class="w-16 pl-2"
					/>
				</div>

				<div class="flex flex-row gap-2">
					<label class="text-lg text-white" for="difficulty">Difficulty:</label>
					<input
						on:input={(e) => inputChangeHandler('difficulty', e, 0, 50)}
						name="difficulty"
						type="number"
						min="0"
						max="50"
						class="w-16 pl-2"
					/>
				</div>
			</div>
		</div>

		<div class="flex gap-8">
			<button
				bind:this={buttonEl}
				{disabled}
				type="submit"
				class="rounded bg-teal-800 px-4 py-2 text-xl text-white shadow-sm active:bg-teal-950 active:shadow-none disabled:bg-stone-600 disabled:active:bg-stone-600 disabled:active:shadow-sm"
				>Roll</button
			>
		</div>
	</div>
</form>

{#if disabled}
	<Tooltip el={buttonEl} direction="top">select a character to roll</Tooltip>
{/if}
