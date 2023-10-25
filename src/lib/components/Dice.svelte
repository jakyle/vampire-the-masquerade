<script lang="ts">
	import bestial from '$lib/assets/bestial.png';
	import critical from '$lib/assets/critical.png';
	import failure from '$lib/assets/failure.png';
	import success from '$lib/assets/success.png';

	export let dieType: 'hunger' | 'normal' = "normal";
	export let roll: number;
</script>

<div class="relative height-[200px] width-[200px] leading-[200px] text-center my-[10px] mx-[40px] flex items-center justify-center">
	<div class="absolute top-0 left-0 h-full w-full transform-[rotateX(45deg)_rotateZ(45deg)] [transform:rotateX(45deg)_rotateZ(45deg)] shadow bg-white"/>
	<span class="relative z-10">{roll}</span>
	<div class="absolute top-[10px] left-[10px] h-[calc(100%-22px)] w-[calc(100%-22px)] border border-orange-500 [transform:rotateX(45deg)_rotateZ(45deg)] shadow bg-white"/>
</div>



<!-- .diamond {
	position: relative;
	height: 200px;
	width: 200px;
	line-height: 200px;
	text-align: center;
	margin: 10px 40px;
  }
  .diamond:before {
	position: absolute;
	content: '';
	top: 0px;
	left: 0px;
	height: 100%;
	width: 100%;
	transform: rotateX(45deg) rotateZ(45deg);
	box-shadow: 0px 0px 12px gray;
  }
  .diamond:after {
	position: absolute;
	top: 10px;
	left: 10px;
	content: '';
	height: calc(100% - 22px);  /* -22px is 2 * 10px gap on either side - 2px border on either side */
	width: calc(100% - 22px);  /* -22px is 2 * 10px gap on either side - 2px border on either side */
	border: 1px solid orange;
	transform: rotateX(45deg) rotateZ(45deg);
  } -->