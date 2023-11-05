<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fly, scale } from 'svelte/transition';
	import Tooltip from './Tooltip.svelte';

	export let shape: 'circle' | 'square' = 'circle';
	export let max = 5;
	export let min = 0;
	export let label = 'values';

	if (min < 0) throw new Error('when using DottedRange, min must be greater than or equal to 0');

	export let value = min;

	const dispatchValueChange = createEventDispatcher<{ change: number }>();

	const options = Array(max)
		.fill(0)
		.map((_, i) => i + 1);

	let elements = options.toRecord((option) => option, null as HTMLElement | null);

	function setValue(val: number) {
		let newValue: number;
		if (val === value) {
			newValue = min;
		} else {
			newValue = val;
		}

		value = newValue;
		dispatchValueChange('change', newValue);
	}

	function inputChangeHandler(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		const numericVal = +e.currentTarget.value;
		let inputVal = isNaN(numericVal) ? value : Math.min(Math.max(numericVal, min), max);
		setValue(inputVal);
	}
</script>

<div class="flex flex-row gap-6 justify-between">
	<label class="text-white" for={label}>{label}</label>
	<div data-direction="horizontal" class="flex flex-col gap-1 data-horizontal:flex-row" id={label}>
		{#each options as option, i}
			<div class="flex items-center gap-3">
				<button
					bind:this={elements[option]}
					class="grid sq-4 cursor-pointer place-items-center data-circle:rounded-full bg-white shadow-sm hover:bg-slate-100 border-stone-900 border"
					class:border-red-950={i < value || value === option}
					data-shape={shape}
					type="button"
					aria-labelledby="{option}-label"
					on:click={() => setValue(option)}
				>
					{#if value === option}
						<div in:scale data-shape={shape} class="sq-3 bg-red-900 data-circle:rounded-full" />
					{/if}

					{#if option < value}
						<div
							in:scale={{ delay: 50 }}
							data-shape={shape}
							class="sq-3 bg-red-400 data-circle:rounded-full"
						/>
					{/if}
				</button>

				{#if elements[option]}
					<Tooltip el={elements[option]} direction="top" delay={800}>
						<div class="bg-gray-50 text-center">
							<span>{option}</span>
						</div>
					</Tooltip>
				{/if}
			</div>
		{/each}
		<input
			class="h-7 w-10 pr-0 text-sm font-extrabold rounded-sm"
			on:input={inputChangeHandler}
			type="number"
			{value}
			{min}
			{max}
		/>
	</div>
</div>
