<script lang="ts">
	// import { onMount } from 'svelte';
	import '../app.css';
	import './useWorker';
	// import { greet, initSync } from 'wasm';
	// import url from 'wasm/wasm_bg.wasm?url';

	let { children } = $props();

	// const showGreeting = () => {
	// 	greet();
	// };

	// onMount(() => {
	// 	fetch(url).then((resp) => {
	// 		resp.arrayBuffer().then((bytes) => {
	// 			initSync(bytes);
	// 		});
	// 	});
	// });
</script>

<!-- <button onclick={showGreeting}>GREET</button> -->

{@render children()}
