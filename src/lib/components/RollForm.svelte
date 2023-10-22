<script context="module" lang="ts">
	const schema = zod.object({
		skill: zod.enum([...skillNames, '']),
		attribute: zod.enum([...attributeNames, '']),
		modifier: zod.number().default(0),
		difficulty: zod.number(),
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

	const dispatchRoll = createEventDispatcher<{
		rollData: RollFormValues;
	}>();

	const { form, data } = createForm<RollFormValues>({
		onSubmit: (values) => dispatchRoll('rollData', values),
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
		selectedAttributeStore.set(values.attribute);
		selectedSkillStore.set(values.skill);
	});

	onDestroy(unsubscribe);
</script>

<form use:form class="flex w-full justify-center">
	<div class="flex w-fit flex-col items-center gap-2 rounded bg-stone-700 px-12 py-8">
		<div class="flex flex-row gap-6">
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

			<div class="flex flex-row gap-2">
				<label class="text-lg text-white" for="skill">Action:</label>
				<select id="action" name="action">
					<option value="roll">roll</option>
					<option value="passive">passive</option>
				</select>
			</div>

			<div class="flex flex-row gap-2">
				<label class="text-lg text-white" for="attribute">Modifier:</label>
				<input name="modifier" type="number" class="w-16 pl-2" />
			</div>

			<div class="flex flex-row gap-2">
				<label class="text-lg text-white" for="attribute">Difficulty:</label>
				<input name="difficulty" type="number" class="w-16 pl-2" />
			</div>
		</div>

		<div class="flex gap-8">
			<button
				type="submit"
				class="rounded bg-teal-800 px-4 py-2 text-xl text-white shadow-sm active:bg-teal-950 active:shadow-none"
				>Roll</button
			>
		</div>
	</div>
</form>
