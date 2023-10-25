<script lang="ts">
	import bestial from '$lib/assets/bestial.png';
	import critical from '$lib/assets/critical.png';
	import failure from '$lib/assets/failure.png';
	import success from '$lib/assets/success.png';
	import Tooltip from './Tooltip.svelte';

	export let dieType: 'hunger' | 'normal' = 'normal';
	export let roll: number; 

	let el: HTMLDivElement;

	$: die = getDieInfo(dieType, roll);

	function getDieInfo(type: typeof dieType, rollNum: number) {
		return {
			color: rollNum === 10 ? 'blue' : rollNum < 6 ? 'red' : 'green',
			src:
				rollNum === 10
					? critical
					: rollNum === 1 && type === 'hunger'
					? bestial
					: rollNum >= 6
					? success
					: failure
		};
	}

	const imageClassMap = {
		[bestial]: 'sq-3 top-1.5',
		[critical]: 'w-[9.5px] h-4 top-[3px]',
		[failure]: 'sq-2 top-1.5',
		[success]: 'w-[8px] h-4 top-[5px]'
	}

</script>

<div bind:this={el} class="relative w-[25px] h-[17.5px] flex items-center justify-center">

	<Tooltip {el} direction="top" delay={600}>
		{roll}
	</Tooltip>

	<div
		class:border-b-red-500={die.color === 'red'}
		class:border-b-green-500={die.color === 'green'}
		class:border-b-blue-500={die.color === 'blue'}
		class:after:border-t-red-500={die.color === 'red'}
		class:after:border-t-green-500={die.color === 'green'}
		class:after:border-t-blue-500={die.color === 'blue'}
		class="flex justify-center items-center w-0 h-0 border-[12.5px] border-transparent border-b-[5px] relative -top-[12.5px]
		after:absolute after:-left-[12.5px] after:top-[5px] after:w-0 after:h-0 after:border-[12.5px] after:border-transparent
		after:border-t-[22.5px]"
	>
		{#if dieType !== 'hunger'}
			<div
				class="w-0 h-0 Z-10 border-[10.5px] border-transparent border-b-[4px] border-b-black relative bottom-[2px]
		after:absolute after:-left-[10.5px] after:z-10 after:top-[3.5px] after:w-0 after:h-0 after:border-[10.5px] after:border-transparent
		after:border-t-[20.5px] after:border-t-black"
			/>
		{/if}
	</div>
	<img
		src={die.src}
		alt="die"
		class="absolute z-20 {imageClassMap[die.src]}" 
	/>
</div>
