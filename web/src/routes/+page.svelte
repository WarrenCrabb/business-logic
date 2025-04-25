<script lang="ts">
	import { onMount } from 'svelte';
	// import * as ace from 'brace';
	// import 'brace/mode/javascript';
	// import 'brace/theme/monokai';
	// // let lib: typeof import('wasm');

	let editorElement: HTMLElement;

	let lang: string = 'json';
	let theme: string = 'chrome';

	onMount(async () => {
		// const editor = ace.edit('javascript-editor');
		// editor.getSession().setMode('ace/mode/javascript');
		// editor.setTheme('ace/theme/monokai');
		// let editor = ace.edit(editorElement);
		// editor.getSession().setMode('ace/mode/' + lang);
		// editor.setTheme('ace/theme/' + theme);
		// lib = await import('wasm');
		// await lib.default();
		// lib.greet();
	});
</script>

<!-- <button onclick={() => lib.greet()}>Greet</button> -->

<!-- <h1>Welcome to SvelteKit</h1> -->
<!-- <p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p> -->

<a href="/editor">Monaco</a>

<div style="width: 800px; height: 800px;">
	<div bind:this={editorElement} style="width: 800px; height: 800px;"></div>
</div>
