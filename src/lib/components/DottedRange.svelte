<script lang="ts">
	import { createRadioGroup, melt } from '@melt-ui/svelte';
	import { createEventDispatcher } from 'svelte';
	import { writable } from 'svelte/store';

	export let max = 5;
	export let min = 0;

	if (min < 0) {
		throw new Error('when using DottedRange, min must be greater than or equal to 0');
	}

	let innerValue: number | undefined = min;

	const dispatchValueChange = createEventDispatcher<{ change: number }>();

	const value = writable(`${innerValue}`);
	const {
		elements: { root, item, hiddenInput },
		helpers: { isChecked },
		options
	} = createRadioGroup({
		orientation: 'horizontal',
		onValueChange: ({ curr, next }) => {
			const isInvalid = +next < min || curr === next;
			if (isInvalid) {
				dispatchValueChange('change', 0);
				innerValue = min;
			} else {
				dispatchValueChange('change', +next);
				innerValue = +next;
			}
			return isInvalid ? '' : next;
		},
		value
	});

	$: updateValueChange(inputVal);
	let inputVal: number = 0;
	function inputChangeHandler(val: string) {
		const numericVal = +val;
		inputVal = isNaN(numericVal) ? +$value : Math.min(Math.max(numericVal, min), max);
	}

	function updateValueChange(val: number) {
		options.onValueChange?.update((fn) => {
			const result = fn!({ curr: $value, next: `${val}` });
			value.set(result);
			return fn;
		});
	}

	const optionsArr = Array(max)
		.fill(0)
		.map((_, i) => `${i + 1}`);
</script>

<div
	use:melt={$root}
	class="flex flex-col gap-1 data-[orientation=horizontal]:flex-row"
	aria-label="View density"
>
	{#each optionsArr as option, i}
		<div class="flex items-center gap-3">
			<button
				use:melt={$item(option)}
				class="grid h-6 w-6 cursor-pointer place-items-center rounded-full bg-white shadow-sm hover:bg-slate-100"
				id={option}
				aria-labelledby="{option}-label"
			>
				{#if $isChecked(option) || i < +$value}
					<div class="h-[1.375rem] w-[1.375rem] rounded-full bg-gray-800" />
				{/if}
			</button>
		</div>
	{/each}
	<input
		on:input={(e) => inputChangeHandler(e.currentTarget.value)}
		value={innerValue}
		type="number"
		{min}
		{max}
	/>
	<input name="number-value" use:melt={$hiddenInput} bind:value={$value} />
</div>
