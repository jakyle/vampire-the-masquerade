<script lang="ts">
	import splatter from '$lib/assets/blood-splatter-one.png';
	import { toImageUrl } from '$lib/util/to-image-url';
	import { rollTension } from '$lib/store/roll-tension';
	import tension from '$lib/assets/tension_1.mp3';
	import { onDestroy } from 'svelte';

	const tensionSound = new Audio(tension);

	let splatterOpacity = "opacity-0";
	let duration = 'duration-0';

	const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
	const rollTensionSub = rollTension.subscribe(async (value) => {
	if (value) {
		tensionSound.play();

		await wait(1900);
		splatterOpacity = "opacity-40";

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
	<div class="w-full h-full {splatterOpacity} {duration}" style="background-image: {toImageUrl(splatter)}"></div>
</div>