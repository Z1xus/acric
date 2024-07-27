<script lang="ts">
	import { onMount } from "svelte";
	import { fade, fly } from "svelte/transition";

	import { base } from "$app/paths";

	let visible = false;

	onMount(() => {
		visible = true;
	});
</script>

<svelte:head>
	<title>contact - acric</title>
</svelte:head>

<div class="text-center min-h-screen flex flex-col justify-center items-center p-4">
	{#if visible}
		<h1 in:fly={{ y: -50, duration: 1000 }} class="text-4xl font-bold mb-6 text-blue-400">
			contact me
		</h1>
		<p in:fade={{ duration: 1000, delay: 300 }} class="mb-8 text-purple-300">
			have questions or want to contribute? reach out to me!
		</p>
		<div
			in:fade={{ duration: 1000, delay: 600 }}
			class="bg-gray-800 p-6 rounded-lg shadow-lg max-w-md mb-8"
		>
			<h2 class="text-2xl font-semibold mb-4 text-green-400">get in touch</h2>
			<ul class="list-none text-left">
				<li class="mb-4">
					<h3 class="text-xl font-semibold mb-2 text-purple-300">github</h3>
					<p>visit the repository to report issues or contribute:</p>
					<a
						href="https://github.com/z1xus/acric"
						target="_blank"
						rel="noopener noreferrer"
						class="text-blue-400 hover:text-blue-300 transition-colors"
					>
						github.com/z1xus/acric
					</a>
				</li>
				<li>
					<h3 class="text-xl font-semibold mb-2 text-purple-300">email</h3>
					<p>for any other inquiries, email me at:</p>
					<a
						href="mailto:z1xus@disroot.org"
						class="text-blue-400 hover:text-blue-300 transition-colors"
					>
						z1xus@disroot.org
					</a>
				</li>
			</ul>
		</div>
		<div
			in:fade={{ duration: 1000, delay: 900 }}
			class="bg-gray-800 p-6 rounded-lg shadow-lg max-w-md mb-8"
		>
			<h2 class="text-2xl font-semibold mb-4 text-green-400">support me</h2>
			<p class="mb-4">if you find acric useful, consider supporting its development:</p>
			<a
				href="https://ko-fi.com/z1xus"
				target="_blank"
				rel="noopener noreferrer"
				class="bg-[#29abe0] text-white font-bold py-2 px-4 rounded-full inline-flex items-center hover:bg-[#14a0d8] transition-colors"
			>
				<svg
					class="w-5 h-5 mr-2"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					fill="currentColor"
				>
					<path
						d="M23.881 8.948c-.773-4.085-4.859-4.593-4.859-4.593H.723c-.604 0-.679.798-.679.798s-.082 7.324-.022 11.822c.164 2.424 2.586 2.672 2.586 2.672s8.267-.023 11.966-.049c2.438-.426 2.683-2.566 2.658-3.734 4.352.24 7.422-2.831 6.649-6.916zm-11.062 3.511c-1.246 1.453-4.011 3.976-4.011 3.976s-.121.119-.31.023c-.076-.057-.108-.09-.108-.09-.443-.441-3.368-3.049-4.034-3.954-.709-.965-1.041-2.7-.091-3.71.951-1.01 3.005-1.086 4.363.407 0 0 1.565-1.782 3.468-.963 1.904.82 1.832 3.011.723 4.311zm6.173.478c-.928.116-1.682.028-1.682.028V7.284h1.77s1.971.551 1.971 2.638c0 1.913-.985 2.667-2.059 3.015z"
					/>
				</svg>
				buy me a coffee
			</a>
		</div>
		<div
			in:fade={{ duration: 1000, delay: 1200 }}
			class="bg-gray-800 p-6 rounded-lg shadow-lg max-w-md"
		>
			<h2 class="text-2xl font-semibold mb-4 text-green-400">contributing</h2>
			<p class="mb-4">contributions are welcome! here's how you can help:</p>
			<ul class="list-disc list-inside text-left">
				<li>report bugs or suggest features through github issues</li>
				<li>submit pull requests for bug fixes or new features</li>
				<li>share acric with others who might find it useful</li>
			</ul>
			<p class="mt-4">
				for major changes, please open an issue first to discuss what you'd like to change.
			</p>
		</div>
		<a
			href="{base}/"
			in:fly={{ y: 50, duration: 1000, delay: 1500 }}
			class="mt-8 bg-gradient-to-r from-blue-500 to-purple-500 text-white font-bold py-3 px-6 rounded-full shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105 hover:from-blue-600 hover:to-purple-600"
		>
			back to home
		</a>
	{/if}
</div>
