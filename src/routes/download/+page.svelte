<script lang="ts">
	import { onMount } from "svelte";
	import { fade, fly } from "svelte/transition";

	let visible = false;

	onMount(() => {
		visible = true;
	});
</script>

<svelte:head>
	<title>download - acric</title>
</svelte:head>

<div class="text-center min-h-screen flex flex-col justify-center items-center p-4">
	{#if visible}
		<h1 in:fly={{ y: -50, duration: 1000 }} class="text-4xl font-bold mb-6 text-blue-400">
			download acric
		</h1>
		<a
			href="https://github.com/z1xus/acric/releases/latest/download/acric.exe"
			target="_blank"
			rel="noopener noreferrer"
			in:fly={{ y: 50, duration: 1000, delay: 300 }}
			class="bg-gradient-to-r from-blue-500 to-purple-500 text-white font-bold py-3 px-6 rounded-full shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105 hover:from-blue-600 hover:to-purple-600 mb-8"
		>
			download latest release
		</a>
		<div
			in:fade={{ duration: 1000, delay: 600 }}
			class="bg-gray-800 p-6 rounded-lg shadow-lg max-w-md text-left"
		>
			<h2 class="text-2xl font-semibold mb-4 text-green-400">quick start</h2>
			<ol class="list-decimal list-inside space-y-2">
				<li>run the executable</li>
				<li>the app will appear in your system tray</li>
				<li>
					use hotkeys:
					<ul class="list-disc list-inside ml-4 mt-2">
						<li><code class="bg-gray-700 px-1 rounded">f6</code>: toggle on/off</li>
						<li>
							<code class="bg-gray-700 px-1 rounded">back mouse button (mb5)</code>: activate
							clicking
						</li>
					</ul>
				</li>
				<li>choose between toggle and hold modes in the tray menu</li>
			</ol>
			<p class="mt-4 text-sm text-gray-400">note: hotkeys are not customizable at the moment</p>
		</div>
		<p in:fade={{ duration: 1000, delay: 900 }} class="mt-8 text-sm text-gray-400 max-w-md">
			for more information, visit the <a
				href="https://github.com/z1xus/acric"
				class="text-blue-400 hover:underline">github repository</a
			>
		</p>
	{/if}
</div>
