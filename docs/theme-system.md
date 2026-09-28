# Arborium themes

Arborium's Rust HTML output contains custom elements such as `<a-k>` (keyword)
and `<a-s>` (string). It does not embed CSS or require JavaScript to display them.
Load a base stylesheet and the color variables for the themes you want.

## Static sites and server-rendered HTML

For example, an Eleventy, mdBook, or other static-site build can emit highlighted
HTML and include these three stylesheets in its page layout:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@arborium/arborium@2.20.0/dist/themes/base.css">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@arborium/arborium@2.20.0/dist/themes/github-light.css">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@arborium/arborium@2.20.0/dist/themes/one-dark.css">
<pre class="code"><code><a-k>fn</a-k> <a-f>main</a-f>() {}</code></pre>
```

`base.css` chooses light or dark colors using the system preference. Set
`data-theme="light"` or `data-theme="dark"` on `<html>` to override it; remove that
attribute to return to the system preference. These rules work without JavaScript.
Include both a light and a dark theme for automatic switching.

Set the background and default text color on your code container too:

```css
.code { background: var(--arb-bg-light); color: var(--arb-fg-light); }
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) .code {
    background: var(--arb-bg-dark); color: var(--arb-fg-dark);
  }
}
:root[data-theme="dark"] .code {
  background: var(--arb-bg-dark); color: var(--arb-fg-dark);
}
```

To host the styles yourself, copy the files from the npm package's `dist/themes/`
directory into your site's static assets. With a bundler, import them directly:

```js
import '@arborium/arborium/themes/base.css';
import '@arborium/arborium/themes/github-light.css';
import '@arborium/arborium/themes/one-dark.css';
```

## Generating and customizing CSS

Theme sources live in `crates/arborium-theme/themes/*.toml`. Run:

```sh
cargo xtask gen
```

This regenerates Rust theme data and CSS in `packages/arborium/src/themes/`.
Building the npm package copies the styles to `dist/themes/`. You only need this
step when developing Arborium or modifying its themes; published packages already
contain the CSS.

Theme files define variables such as `--arb-k-light`, `--arb-s-dark`,
`--arb-bg-dark`, and `--arb-fg-light`. The suffix is determined by the TOML
`variant` (`light` or `dark`). The short tag is part of the variable name:
`--arb-k-dark`, not `--arb-keyword-dark`. Weight, style, and decoration variables
use suffixes such as `--arb-k-dark-weight`.

For custom renderers, use `getTagForCapture(capture)` from `@arborium/arborium`.
For example, `function.call` maps to `f`, and `keyword.function` maps to `k`.
It returns `null` for captures with no styling. The exported `highlights` list
contains legacy subcategory tags and is not a substitute for this mapping.

## IIFE and rustdoc

`base-rustdoc.css` is for a page that loads **one active theme at a time**. It
uses dark variables when present, then falls back to light variables. It does not
perform system-preference switching by itself. Loading both light and dark theme
files with it makes the dark variables take precedence.

The IIFE loader uses that base stylesheet and changes the loaded theme when the
system preference or page theme changes. On rustdoc pages it maps the `light`,
`dark`, and `ayu` settings to the corresponding `rustdoc-*.css` themes. Custom
light/dark choices can be set with `data-theme-light` and `data-theme-dark` on the
script element. For static output with both themes present, use `base.css`.
