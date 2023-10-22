<script lang="ts">
	export let value: number;
	export let label: string;
	export let min: number = 0;
</script>

<div class="flex w-full flex-row items-center gap-4 rounded-sm border border-white p-2 py-4">
	<label
		for={label}
		class="mb-2 block w-40 text-sm font-medium uppercase text-stone-900 dark:text-white"
		>{label}:</label
	>
	<input
		on:input
		id={label}
		autocomplete="off"
		{min}
		max="5"
		type="range"
		step="1"
		bind:value
		class="w-full cursor-pointer rounded-lg bg-stone-200 dark:bg-stone-700"
	/>
	<input on:input bind:value class="w-6 rounded bg-white pl-1.5 font-bold text-black" />
</div>
