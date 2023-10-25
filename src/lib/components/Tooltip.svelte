<script lang="ts">
	import { onDestroy } from 'svelte';
	import { portal } from 'svelte-portal';
	import { innerHeight, innerWidth } from '$lib/store/window';

	export let el: HTMLElement | undefined;
	export let direction: 'left' | 'right' | 'top' | 'bottom' = 'right';
	export let delay = 500;

	$: isRight = direction === 'right';
	$: isLeft = direction === 'left';
	$: isTop = direction === 'top';
	$: isBottom = direction === 'bottom';

	$: if (el) {
		el.addEventListener('mouseenter', mouseEnter);
		el.addEventListener('mouseleave', mouseLeave);
	} else {
		isHovered = false;
		isVisible = false;
	}

	let isVisible = false;
	let isHovered = false;
	let toolTipEl: HTMLDivElement;

	let x: number;
	let y: number;

	innerWidth.subscribe((w) => (x = w / 2));
	innerHeight.subscribe((h) => (y = h / 2));

	let timeout: NodeJS.Timeout | undefined;
	function mouseEnter(event: Event) {
		clearTimeout(timeout!);
		event.preventDefault();
		timeout = setTimeout(() => {
			updateToolTipPosition();
			isVisible = true;
		}, delay);
		isHovered = true;
	}

	function updateToolTipPosition() {
		const { left, right, top, bottom, height, width } = el!.getBoundingClientRect();
		const basePx = 12;

		const centerOffsetX = toolTipEl.clientWidth / 2;
		const centerOffsetY = toolTipEl.clientHeight / 2;

		x =
			direction === 'left'
				? left - (toolTipEl.clientWidth + basePx)
				: direction === 'right'
				? right + basePx
				: left + width / 2 - centerOffsetX - 1.5;

		y =
			direction === 'top'
				? top - toolTipEl.clientHeight - basePx
				: direction === 'bottom'
				? bottom + basePx
				: top + height / 2 - centerOffsetY;
	}

	function mouseLeave() {
		clearTimeout(timeout!);
		isVisible = false;
		timeout = setTimeout(() => {
			isHovered = false;
		}, 150);
	}

	onDestroy(() => {
		el!.removeEventListener('mouseenter', mouseEnter);
		el!.removeEventListener('mouseleave', mouseLeave);
		isVisible = false;
		isHovered = false;
	});
</script>

{#if isHovered}
	<div
		dir={isLeft ? 'rtl' : 'ltr'}
		bind:this={toolTipEl}
		use:portal={'body'}
		style:left={`${x}px`}
		style:top={`${y}px`}
		class:opacity-100={isVisible}
		class:opacity-0={!isVisible}
		class:z-50={isVisible}
		class:after:top-[50%]={isRight || isLeft}
		class:after:-translate-y-[50%]={isRight || isLeft}
		class:after:border-y-transparent={isRight || isLeft}
		class:min-h-8={isRight || isLeft}
		class:after:left-[50%]={isTop || isBottom}
		class:after:-translate-x-[50%]={isTop || isBottom}
		class:after:border-x-transparent={isTop || isBottom}
		class:min-w-8={isTop || isBottom}
		class:after:top-[100%]={isTop}
		class:after:border-b-transparent={isTop}
		class:after:bottom-[100%]={isBottom}
		class:after:border-t-transparent={isBottom}
		class:after:right-[100%]={isRight}
		class:after:border-l-transparent={isRight}
		class:after:left-[100%]={isLeft}
		class:after:border-r-transparent={isLeft}
		class:duration-150={!isVisible}
		class:duration-300={isVisible}
		class="fixed p-1 bg-stone-50 border-stone-900 border-2 rounded-lg w-auto after:absolute after:border-[10px] after:border-stone-50 pointer-events-none transition-opacity"
	>
		<slot />
	</div>
{/if}
