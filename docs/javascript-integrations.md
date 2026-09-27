# JavaScript integrations

## Capture names and themes

Use `getTagForCapture` for the short tag emitted by Arborium's Rust renderer:

```js
import { getTagForCapture } from '@arborium/arborium';
getTagForCapture('function.call'); // 'f'
getTagForCapture('keyword.function'); // 'k'
getTagForCapture('spell'); // null (no styling)
```

This handles aliases and unknown subcategories like `function.custom`. The
`highlights` metadata contains legacy subcategory tags, which may differ from the
renderer. Using those tags directly can leave Elixir modules or other captures
unstyled.

## Local npm packages, Node, and offline builds

Install the main package and every grammar your application needs at matching
versions. Configure all four resolvers to avoid network requests. Node can load
WASM bytes from the installed packages:

```js
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { highlight, setConfig } from '@arborium/arborium';
import * as host from '@arborium/arborium/arborium_host.js';
import * as jsonGrammar from '@arborium/json';

setConfig({
  resolveHostJs: () => host,
  resolveHostWasm: () => readFile(fileURLToPath(
    import.meta.resolve('@arborium/arborium/arborium_host_bg.wasm'))),
  resolveJs: ({ language }) => {
    if (language === 'json') return jsonGrammar;
    throw new Error(`Grammar not installed: ${language}`);
  },
  resolveWasm: ({ language }) => {
    if (language !== 'json') throw new Error(`Grammar not installed: ${language}`);
    return readFile(fileURLToPath(import.meta.resolve('@arborium/json/grammar_bg.wasm')));
  },
});
const html = await highlight('json', '{"answer":42}');
```

In a browser bundler, use the same explicit JavaScript import map and have the
WASM resolvers return your bundler's asset URLs (or fetched bytes). Exact asset
import syntax depends on the bundler. For a plain static server, return modules
and WASM URLs beneath your own assets directory. Default CDN imports carry both
Vite and Webpack ignore hints; explicit resolvers avoid bundler analysis of a
variable remote URL altogether. Simply importing a grammar does not register it.

`loadGrammar(...).parse(source)` and session parsing return **UTF-16** offsets for
JavaScript string slicing and DOM ranges. Rust's internal host interface uses
UTF-8 bytes; do not mix the two interfaces. Always call `session.free()` when an
incremental session is no longer needed.

## CSS Custom Highlight API

For applications that keep plain text in the DOM, the existing span API can feed
[CSS Custom Highlights](https://www.w3.org/TR/css-highlight-api-1/). This minimal
example assumes the code element contains one Text node and owns the registry
names it uses:

```js
import { loadGrammar, getTagForCapture } from '@arborium/arborium';

const code = document.querySelector('pre code');
const text = code.firstChild;
if (!(text instanceof Text) || code.childNodes.length !== 1) {
  throw new Error('Expected one plain Text node');
}
const grammar = await loadGrammar('json');
if (!grammar) throw new Error('JSON grammar could not be loaded');
const groups = new Map();
for (const span of grammar.parse(text.data).spans) {
  const tag = getTagForCapture(span.capture);
  if (!tag) continue;
  const range = new Range();
  range.setStart(text, span.start);
  range.setEnd(text, span.end);
  const ranges = groups.get(tag) ?? [];
  ranges.push(range);
  groups.set(tag, ranges);
}
for (const [tag, ranges] of groups) {
  CSS.highlights.set(`arborium-${tag}`, new Highlight(...ranges));
}
// On teardown: for (const tag of groups.keys()) CSS.highlights.delete(`arborium-${tag}`);
```

Load theme variables and define selectors such as:

```css
::highlight(arborium-s) { color: var(--arb-s-dark); }
::highlight(arborium-pr) { color: var(--arb-pr-dark); }
::highlight(arborium-n) { color: var(--arb-n-dark); }
::highlight(arborium-co) { color: var(--arb-co-dark); }
```

Custom highlights support a limited set of text-paint properties, not layout or
font-weight changes. Check `CSS.highlights` and `Highlight` before using this API.
Applications with multiple code blocks, overlapping captures, language injections,
or edited DOM trees must manage range lifetimes, priorities, and registry ownership.
The built-in HTML renderer handles injection and overlap resolution automatically.

## Terminal recordings with ANSI colors

ANSI-colored terminal output already contains its styling. Convert it using an
ANSI decoder such as [ansi_up](https://github.com/drudru/ansi_up), instead of
passing the escape sequences to a programming-language grammar:

```js
import { AnsiUp } from 'ansi_up';
const converter = new AnsiUp();
const html = converter.ansi_to_html('\u001b[31merror: <missing file>\u001b[0m');
```

For an Eleventy filter, create a new converter for each independent recording.
Keep `escape_html` enabled (the default); emit the returned HTML inside a `<pre>`.
This decoder handles SGR colors and supported terminal links; it is not a terminal
emulator for cursor movement. Arborium's `AnsiHighlighter` performs the opposite
operation: source code to ANSI-colored terminal output.
