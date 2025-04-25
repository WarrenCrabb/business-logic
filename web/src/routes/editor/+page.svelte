<script lang="ts">
	import { editor } from 'monaco-editor';
	import { onDestroy, onMount } from 'svelte';
	let lib: typeof import('wasm');

	let editorContainer: HTMLElement;

	let my_editor: editor.IStandaloneCodeEditor;

	let v = $state<string>('');

	const getVal = () => {
		const _v = my_editor?.getValue({
			preserveBOM: true,
			lineEnding: '\n'
		});
		const r = lib.eval_business_logic(_v);

		console.log('res', r);

		v = r;
	};

	onMount(async () => {
		// const monaco = await import('monaco-editor');
		lib = await import('wasm');
		await lib.default();

		// editor.

		my_editor = editor.create(editorContainer, {
			value: "console.log('Hello, world!');",
			language: 'javascript',
			theme: 'vs-dark'
		});
		// my_editor.getValue();
		// 	monaco.languages.register({ id: 'mySpecialLang' });
		// 	monaco.languages.setMonarchTokensProvider('mySpecialLang', {
		// 		tokenizer: {
		// 			root: [
		// 				[
		// 					/[a-z_$][\w$]*/,
		// 					{
		// 						cases: {
		// 							'@keywords': 'keyword',
		// 							'@default': 'identifier'
		// 						}
		// 					}
		// 				],
		// 				[/\d+/, 'number'],
		// 				[/[{}]/, '@brackets']
		// 			]
		// 		},
		// 		keywords: ['custom', 'keyword']
		// 	});
	});

	onDestroy(() => {
		my_editor?.dispose();
	});
</script>

<div bind:this={editorContainer} style="height: 500px;"></div>

<button onclick={getVal}>TEXT</button>

<div>
	<pre>
	<code>{v}</code>
  </pre>
</div>
