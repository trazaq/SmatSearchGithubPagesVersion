<script lang="ts">
	import { PUBLIC_ENV } from '$env/static/public';

	export let data;
	import SearchThreads from '../SearchThreads.svelte';
	let color: string;
	if (PUBLIC_ENV.toLowerCase().includes('prod')) {
		color = 'springgreen';
	} else {
		color = 'navajowhite';
	}
</script>

<svelte:head>
	<title>Cloverleaf Message Search</title>
	<meta name="description" content="" />
	<style>
		root {
			background-color: #242424;
			color: rgba(255, 255, 255, 0.87);
		}
		body {
			background-color: #242424;
			color: rgba(255, 255, 255, 0.87);
		}
		input {
			color-scheme: light dark;
		}
	</style>
</svelte:head>

<div class="app">
	<h1 style="color: {color}">Cloverleaf Message Search ({PUBLIC_ENV})</h1>
	<br />
	<br />
	<SearchThreads all_threads={data.all_threads} />
	<br />
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.app {
		max-width: 100%;
		min-height: 100%;
		margin: 0 auto;
		padding: 2rem;
		text-align: center;
	}
</style>
