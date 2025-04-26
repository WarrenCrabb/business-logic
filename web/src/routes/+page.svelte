<script lang="ts">
	import { editor, languages } from 'monaco-editor';
	import { onDestroy, onMount } from 'svelte';
	let lib: typeof import('wasm');

	let editorContainer: HTMLElement;

	let my_editor: editor.IStandaloneCodeEditor;

	let results = $state<Array<string>>([]);

	const getVal = () => {
		try {
			const _v = my_editor?.getValue({
				preserveBOM: true,
				lineEnding: '\n'
			});
			const r = lib.eval_business_logic(_v);

			// const res = new Date().toUTCString();

			results.push(`> ${r}`);
		} catch (error) {
			results.push(error as string);
		}
	};

	const s = `
initiate make_adder align strategy a as
  deliver plan b to 
    synergize a + b
  

initiate add_five with make_adder leveraging 5

actualize res = add_five utilize 2

touch_base invoke res
  `;

	onMount(async () => {
		lib = await import('wasm');
		await lib.default();

		my_editor = editor.create(editorContainer, {
			// value: 'print invoke "Hello, World"',
			value: s,
			language: 'business_logic',
			theme: 'vs-dark'
		});

		languages.register({ id: 'business_logic' });

		languages.setMonarchTokensProvider('business_logic', {
			operators: ['=', '>', '<', '!', '==', '<=', '>=', '!=', '&&', '||', '+', '-', '*', '/', '%'],

			keywords: [
				'unrealized',
				'let',
				'actualize',
				'initiate',
				'return',
				'synergize',
				'execute',
				'deliver',
				'fn',
				'plan',
				'strategize',
				'strat',
				'blueprint',
				'leverage',
				'leveraging',
				'utilize',
				'engage',
				'activate',
				'invoke',
				'while',
				'until',
				'for',
				'align',
				'with',
				'to',
				'as',
				'if',
				'evaluate',
				'elif',
				're_evaluate',
				'pivot',
				'else',
				'is',
				'equals',
				'less',
				'below',
				'above',
				'exceeds',
				'achieving',
				'productive',
				'unproductive',
				'ineffectual',
				'add',
				'value_add',
				'increase',
				'plus',
				'reduce',
				'streamline',
				'subtract',
				'cut',
				'multiply',
				'amplify',
				'boost',
				'divide',
				'disrupt',
				'modulo',
				'remainder',
				'surplus',
				'not',
				'true',
				'actionable',
				'false',
				'headwinds',
				'empty',
				'end',
				'print',
				'touch_base'
			],
			symbols: /[=><!~?:&|+\-*\/\^%]+/,
			escapes: /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

			tokenizer: {
				root: [
					[
						/[a-z_$][\w$]*/,
						{
							cases: {
								'@keywords': 'keyword',
								'@default': 'identifier'
							}
						}
					],
					[/[A-Z][\w\$]*/, 'type.identifier'],
					[/\d+/, 'number'],
					// [/[{}]/, '@brackets'],
					[/"([^"\\]|\\.)*$/, 'string.invalid'], // non-teminated string
					[/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],

					[/@symbols/, { cases: { '@operators': 'operator', '@default': '' } }],
					// whitespace
					{ include: '@whitespace' }
				],
				comment: [
					[/[^\/*]+/, 'comment'],
					[/\/\*/, 'comment', '@push'], // nested comment
					['\\*/', 'comment', '@pop'],
					[/[\/*]/, 'comment']
				],
				string: [
					[/[^\\"]+/, 'string'],
					[/@escapes/, 'string.escape'],
					[/\\./, 'string.escape.invalid'],
					[/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }]
				],

				whitespace: [
					[/[ \t\r\n]+/, 'white'],
					[/\/\*/, 'comment', '@comment'],
					[/\/\/.*$/, 'comment']
				]
			}
		});
	});

	onDestroy(() => {
		my_editor?.dispose();
	});
</script>

<div class="w-full" bind:this={editorContainer} style="height: 500px;"></div>

<button class="hover:cursor-pointer" aria-label="run" onclick={getVal}>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		viewBox="0 0 24 24"
		stroke-width="0.5"
		stroke="currentColor"
		class="size-6 fill-green-500 hover:fill-green-600"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"
		/>
	</svg>
</button>

<div class="w-full border border-slate-300 bg-slate-200 px-4 py-2">
	<ul>
		{#each results as result}
			<li>
				{result}
			</li>
		{/each}
	</ul>
	<!-- <pre>
	<code>{results}</code>
  </pre> -->
</div>
