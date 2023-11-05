<script lang="ts">
	import { rollTension } from '$lib/store/roll-tension';
	import tension from '$lib/assets/tension_1.ogg';
	import { onDestroy } from 'svelte';
	import { toImageUrl } from '$lib/util/to-image-url';
	import splatter from '$lib/assets/blood-splatter-one.png';

	const tensionSound = new Audio(tension);

	let splatterOpacity = 'opacity-0';
	let duration = 'duration-0';

	const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
	const rollTensionSub = rollTension.subscribe(async (value) => {
		if (value) {
			tensionSound.currentTime = 0;
			tensionSound.play();
			duration = 'duration-1000';
			splatterOpacity = 'opacity-10';
			await wait(1750);
			duration = 'duration-500';
			splatterOpacity = 'opacity-50';
			await wait(600);
			duration = 'duration-1000';
			splatterOpacity = 'opacity-0';

			await wait(500);
			duration = 'duration-0';
			rollTension.set('');
		}
	});
	onDestroy(rollTensionSub);
</script>

<div class="absolute w-full h-full z-10 pointer-events-none">
	<div
		class="w-full h-full {splatterOpacity} {duration}"
		style="background-image: url('{splatter}')"
	/>
</div>
