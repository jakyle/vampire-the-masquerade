<script lang="ts">
	import bestial from '$lib/assets/bestial.png';
	import critical from '$lib/assets/critical.png';
	import failure from '$lib/assets/failure.png';
	import success from '$lib/assets/success.png';
	import DiamondIcon from './Icons/DiamondIcon.svelte';
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
		[bestial]: 'w-2/6 h-2/6',
		[critical]: 'w-2/6 h-5/12',
		[failure]: 'w-1/4 h-1/4',
		[success]: 'w-3/12'
	};
</script>

<div bind:this={el} class="relative w-12 h-12 flex items-center justify-center isolation">
	<Tooltip {el} direction="top" delay={600}>
		{roll}
	</Tooltip>

	<DiamondIcon
		class="w-full h-full absolute {die.color === 'blue'
			? 'fill-blue-800'
			: die.color === 'red'
			? 'fill-red-800'
			: 'fill-green-800'}"
	/>
	{#if dieType !== 'hunger'}
		<DiamondIcon class="absolute w-[85%] h-[85%] absolute z-10 fill-black" />
	{/if}
	<img src={die.src} alt="die" class="relative z-20 {imageClassMap[die.src]} " />
</div>
