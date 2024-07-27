<script lang="ts">
	import { onMount } from "svelte";
	import { fade, fly, scale } from "svelte/transition";
	
	import { base } from "$app/paths";

	let features = ["realistic cps calculation", "hotkey support", "fatigue simulation", "tray icon"];

	let visible = false;
	let chartExpanded = false;
	let showFormula = false;

	onMount(() => {
		visible = true;
	});

	function toggleChart() {
		chartExpanded = !chartExpanded;
	}

	function toggleFormula() {
		showFormula = !showFormula;
	}
</script>

<svelte:head>
	<title>acric - your realistic autoclicker</title>
</svelte:head>

<div class="text-center min-h-screen flex flex-col justify-center items-center p-4">
	{#if visible}
		<img
			in:fly={{ y: -50, duration: 1000 }}
			class="w-32 h-32 mx-auto mb-6 animate-pulse floating"
			src={"./acric.svg"}
			alt="acric logo"
		/>
		<h1
			in:fly={{ y: 50, duration: 1000, delay: 300 }}
			class="text-4xl font-bold mb-4 text-blue-400"
		>
			acric
		</h1>
		<p in:fade={{ duration: 1000, delay: 600 }} class="text-xl mb-8 text-purple-300">
			your most realistic, fully-external autoclicker
		</p>

		<div in:fade={{ duration: 1000, delay: 900 }} class="mb-8">
			<h2 class="text-2xl font-semibold mb-4 text-green-400">features</h2>
			<ul class="list-none">
				{#each features as feature, i}
					<li
						in:fly={{ x: 50, duration: 500, delay: 1200 + i * 100 }}
						class="mb-2 bg-gray-800 rounded-lg p-2 shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105"
					>
						{feature}
					</li>
				{/each}
			</ul>
		</div>

		<div in:fade={{ duration: 1000, delay: 1800 }} class="mb-8 w-full max-w-2xl">
			<h2 class="text-2xl font-semibold mb-4 text-green-400">why it's so realistic</h2>
			<div class="flex flex-col items-center">
				<ul
					class="list-disc list-inside text-left mb-4 transition-all duration-300 ease-in-out"
					class:w-64={!chartExpanded}
					class:w-full={chartExpanded}
				>
					<li>fatigue system</li>
					<li>random delays</li>
					<li>occasional missclicks</li>
					<li>double clicks</li>
					<li>
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<span
							class="cursor-pointer text-blue-400 hover:text-blue-300 transition-colors"
							on:click={toggleFormula}
						>
							complex cps calculation formula
						</span>
						{#if showFormula}
							<p in:fade={{ duration: 300 }} class="ml-4 mt-2 text-sm text-gray-400">
								a combination of base rate, sinusoidal variation and random noise
							</p>
						{/if}
					</li>
				</ul>
				<p class="mt-4 mb-2">
					cps chart for reference (click to {chartExpanded ? "minimize" : "expand"}):
				</p>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class="bg-gray-700 mt-2 flex items-center justify-center cursor-pointer transition-all duration-300 ease-in-out overflow-hidden"
					class:w-full={chartExpanded}
					class:h-42={chartExpanded}
					class:w-64={!chartExpanded}
					class:h-18={!chartExpanded}
					on:click={toggleChart}
				>
					{#if chartExpanded}
						<div
							in:scale={{ duration: 300, start: 0.5 }}
							class="w-full h-full flex items-center justify-center"
						>
							<img src="mercury_VhytMK4Abg.png" alt="cps chart" />
						</div>
					{:else}
						<div class="w-full h-full flex items-center justify-center">
							<img src="mercury_VhytMK4Abg.png" alt="cps chart" />
						</div>
					{/if}
				</div>
			</div>
		</div>

		<div in:fade={{ duration: 1000, delay: 2100 }} class="mb-8">
			<p class="text-sm bg-gray-800 p-4 rounded-lg">
				disclaimer: use it at your own risk. autoclicking is generally considered cheating in games.
			</p>
		</div>

		<a
			href="{base}/download"
			in:fly={{ y: 50, duration: 1000, delay: 2400 }}
			class="bg-gradient-to-r from-blue-500 to-purple-500 text-white font-bold py-3 px-6 rounded-full shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105 hover:from-blue-600 hover:to-purple-600"
		>
			download now
		</a>
	{/if}
</div>
