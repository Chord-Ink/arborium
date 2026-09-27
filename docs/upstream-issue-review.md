# Upstream issue review

Reviewed all **107 issues (31 open, 76 closed)** returned by the upstream GitHub
issue API on 2026-09-27, including descriptions and discussion. This records the
disposition in the Chord-Ink fork; it does not close or comment on upstream issues.

The selection prioritizes reproducible failures, missing grammars with usable
upstream sources, and integrations that fit the existing runtime. Component-model
ABI work, a pure-Rust rewrite, and an unreproduced Android browser-process crash
are explicitly left open. ANSI decoding and CSS Custom Highlights have integration
recipes rather than new core rendering backends.

## Issue-by-issue disposition

| Issue | Upstream state | Disposition | Evidence / rationale |
|---|---|---|---|
| [#213: arborium-zsh missing `scanner.c`?](https://github.com/bearcove/arborium/issues/213) | open | Fixed | Updated Zsh to 7a593401 and included its external scanner; heredoc regression. |
| [#212: arborium-sysroot's -l static=arborium_sysroot is never emitted to the final wasm link, leaving stderr undefined even though a stub exists](https://github.com/bearcove/arborium/issues/212) | open | Verified fixed | Direct tree-sitter debug WASM consumer links and executes with zero host imports; protected by check_wasm.py. |
| [#209: [Language Support Request] Actionscript](https://github.com/bearcove/arborium/issues/209) | open | Implemented | Added ActionScript grammar, queries, sample, and upstream corpus; escaped the Unicode-escape regex for current tree-sitter. |
| [#207: arborium-just 2.18.1 fails to compile with zig (NDEBUG / assertions)](https://github.com/bearcove/arborium/issues/207) | open | Fixed | Removed Just’s obsolete NDEBUG rejection; its custom assertions do not depend on the standard assert macro. |
| [#206: arborium-perl: please bump the vendored grammar — upstream fixed the bsearch issues from #154](https://github.com/bearcove/arborium/issues/206) | open | Fixed | Updated Perl to f678e356, including upstream scanner/header fixes; EOF-comment regression and upstream corpus. |
| [#203: A lil more info on CSS build process](https://github.com/bearcove/arborium/issues/203) | open | Documented | Rewrote theme-system.md with static-site CSS, local assets, actual short variable names, generation, and base stylesheet selection. |
| [#198: Language Support: Slang](https://github.com/bearcove/arborium/issues/198) | open | Implemented | Added Slang grammar, its own highlight queries, source compatibility adjustment for the bundled C++ grammar, and corpus. |
| [#197: demo file breaks batch highlighter](https://github.com/bearcove/arborium/issues/197) | open | Fixed | Batch accepts otherwise unsupported command arguments locally, preserving highlighting after prompt escape sequences. |
| [#196: Language support: Add support for "inline" markdown](https://github.com/bearcove/arborium/issues/196) | open | Fixed | Added markdown_inline grammar/plugin and injection feature wiring; bold, italic, code, and links tested with lang-markdown alone. |
| [#195: Koto language support (small Rust scripting language)](https://github.com/bearcove/arborium/issues/195) | open | Implemented | Added Koto grammar, queries, sample, and upstream corpus. |
| [#194: scss highlight is quite broken](https://github.com/bearcove/arborium/issues/194) | open | Fixed | Switched SCSS to tree-sitter-grammars at 2ef6d42e; mixin regression and upstream corpus. |
| [#193: elixir does not highlight very well](https://github.com/bearcove/arborium/issues/193) | open | Verified / documented | Native semantic tags for defmodule, import, and module names pass. Added the exact JS renderer mapping to avoid legacy-tag styling errors. |
| [#192: clojure does not highlight very well](https://github.com/bearcove/arborium/issues/192) | open | Fixed | Added Clojure symbols, function calls/definitions, namespace, special-form keywords, and bracket captures. |
| [#183: haskell has been broken since 2.14.0](https://github.com/bearcove/arborium/issues/183) | closed | Retained closed | Haskell’s earlier upstream fix remains; the all-grammar query check now covers it. |
| [#182: fsharp has been broken since 2.14.0](https://github.com/bearcove/arborium/issues/182) | open | Fixed | Corrected impossible F# query patterns; native query compilation and WASM namespace reproduction pass. |
| [#175: [Grammar Request] protobuf support](https://github.com/bearcove/arborium/issues/175) | closed | Retained closed | Protocol Buffers grammar is already bundled; included in query/WASM checks. |
| [#173: Grammar resolution on web](https://github.com/bearcove/arborium/issues/173) | closed | Retained closed | Added Webpack ignore hints alongside Vite and documented explicit local host/grammar resolvers; a general bundler plugin remains outside this change. |
| [#172: No convenient way to remove GPL parsers](https://github.com/bearcove/arborium/issues/172) | open | Fixed | Added all-permissive-languages and compatibility aliases; corrected generated docs to state default = []; the permissive bundle excludes Nginx (GPL) and Uiua (MPL). |
| [#170: (Feature Request) Highlighting ANSI input?](https://github.com/bearcove/arborium/issues/170) | open | Documented alternative | ANSI input is already styled terminal output. Added an ansi_up integration recipe; a terminal decoder is kept outside the grammar engine. |
| [#166: 2.14 crashes in node with an “unreachable” assertion when parsing XML](https://github.com/bearcove/arborium/issues/166) | closed | Retained closed | XML is bundled and passes query compilation and WASM initialization; upstream scanner fix retained. |
| [#164: COBOL doesn't build on windows msvc](https://github.com/bearcove/arborium/issues/164) | closed | Retained closed | Retained MSVC scanner changes. Also fixed the independent COBOL EOF scanner loop identified in PR #211. |
| [#160: Language list on website should be Ctrl+F'able](https://github.com/bearcove/arborium/issues/160) | closed | Retained closed | Language browsing/search is an existing demo concern; no new reproducible defect in this report. |
| [#159: Plugin architecture is extremely difficult to use in non-browser WASM VMs](https://github.com/bearcove/arborium/issues/159) | open | Deferred: architecture | A portable component/WASI plugin ABI is a separate backend and release contract. Existing plugins intentionally use wasm-bindgen; no untested ABI shim was introduced. |
| [#157: Remove dangling component model stuff](https://github.com/bearcove/arborium/issues/157) | closed | Retained closed | Superseded by the explicit non-browser ABI request #159; no component-model backend claimed. |
| [#156: Building on macOS requires installing llvm](https://github.com/bearcove/arborium/issues/156) | open | Fixed | Plugin preflight compiles a WASM C object with cc-rs compiler selection and reports Homebrew LLVM setup when the compiler lacks WASM support. |
| [#155: `cargo xtask gen` usage is confusing](https://github.com/bearcove/arborium/issues/155) | closed | Retained closed | Committed Cargo build inputs and shared prerequisite generation remove the reported bootstrap problem. |
| [#154: arborium-perl: Linking error with MSVC](https://github.com/bearcove/arborium/issues/154) | closed | Retained closed | Superseded by the upstream Perl refresh in #206, including the renamed search helper. |
| [#152: request: expose getTagForCapture in js api](https://github.com/bearcove/arborium/issues/152) | open | Implemented | Exported getTagForCapture, generated from the Rust capture mapping and shared prefix fallbacks; tested aliases, unknown captures, and prototype-like names. |
| [#151: most of iife demo is invisible on light theme](https://github.com/bearcove/arborium/issues/151) | closed | Retained closed | Retained existing IIFE theme behavior; new theme documentation explains default foreground/background and base CSS selection. |
| [#149: Deno: Cannot load grammars. Likely due to https import?](https://github.com/bearcove/arborium/issues/149) | closed | Retained closed | Node/Deno resolver use is documented; loader already uses globalThis rather than window. |
| [#148: Cannot use the esm module in node. "window is not defined"](https://github.com/bearcove/arborium/issues/148) | closed | Retained closed | Loader uses globalThis; JavaScript type checks pass. Added an offline Node resolver example. |
| [#147: WebAssembly Text Format support](https://github.com/bearcove/arborium/issues/147) | open | Implemented | Added WebAssembly text (wat/wast) grammar, queries, sample, and upstream corpus. |
| [#146: Crystal language support](https://github.com/bearcove/arborium/issues/146) | open | Implemented | Added Crystal grammar, scanner, queries, sample, corpus, and corrected the WASM sysroot static_assert macro. |
| [#145: highlighting source code in a table](https://github.com/bearcove/arborium/issues/145) | closed | Retained closed | Existing session/span APIs support custom line/table renderers; no built-in table renderer required. |
| [#143: npm `@arborium/arborium` crate: usage without CDN](https://github.com/bearcove/arborium/issues/143) | closed | Retained closed | Documented all four local resolvers, installed grammar imports, and WASM byte loading without a CDN. |
| [#142: Add arborium-ratatui adapter crate](https://github.com/bearcove/arborium/issues/142) | open | Implemented | Added arborium-ratatui with owned Line output, overlap resolution, Unicode/blank-line preservation, and style modifiers. |
| [#135: @arborium/arborium: Allow configuring logger](https://github.com/bearcove/arborium/issues/135) | closed | Retained closed | Configurable logger already exists in ArboriumConfig and loader. |
| [#133: @arborium/arborium: IIFE config handling is broken in many ways](https://github.com/bearcove/arborium/issues/133) | closed | Retained closed | Existing getConfig/setConfig and IIFE configuration retained; no new reproducer. |
| [#131: Consider including background and foreground base colours within the npm package CSS dist as variables](https://github.com/bearcove/arborium/issues/131) | closed | Retained closed | Theme CSS already provides --arb-bg-* and --arb-fg-*; documented container styling. |
| [#128: highlight() output includes trailing newline](https://github.com/bearcove/arborium/issues/128) | closed | Retained closed | Trailing-newline trimming remains deliberate in HTML rendering; Ratatui adapter separately preserves source lines. |
| [#125: Tracking: Grammars with C symbols that break WASM browser compatibility](https://github.com/bearcove/arborium/issues/125) | open | Verified fixed | check_wasm.py links and executes all bundled languages and rejects every host import; CI now tests a final module rather than only rlibs. |
| [#122: Regression: JS Spans contain UTF-8 offsets again](https://github.com/bearcove/arborium/issues/122) | closed | Retained closed | Existing UTF-16 public / UTF-8 host split retained and documented. |
| [#118: All punctuation uses the `punctuation.special` color](https://github.com/bearcove/arborium/issues/118) | closed | Retained closed | Rust maps punctuation subcategories to the punctuation slot; JS helper now uses the same mapping. |
| [#115: Syntax highlighting generates malformed HTML with stray closing tags](https://github.com/bearcove/arborium/issues/115) | closed | Retained closed | Reported malformed HTML was attributed upstream to marq, not Arborium; no Arborium fix justified. |
| [#112: Remove KDL support](https://github.com/bearcove/arborium/issues/112) | closed | Retained closed | Language-removal policy request, not a technical defect; no grammar removed. |
| [#110: Building the CLI standalone](https://github.com/bearcove/arborium/issues/110) | open | Fixed; Nix execution unverified | Committed CLI lockfile and all build inputs; added standalone nix/package.nix, flake package output, and Cargo/Nix instructions. Nix is not installed in the review environment. |
| [#109: Property name collisions in CSS themes](https://github.com/bearcove/arborium/issues/109) | closed | Retained closed | Current property and punctuation tags differ (pr/p); mapping tests retained. |
| [#105: New Function to Return `LanguageFn` by Language Name](https://github.com/bearcove/arborium/issues/105) | closed | Retained closed | Generated get_language API already exists and is used by new language regressions. |
| [#104: Unable to build according to instructions](https://github.com/bearcove/arborium/issues/104) | closed | Retained closed | Build documentation now includes ordinary locked Cargo builds and macOS LLVM preflight. |
| [#101: Pug language support](https://github.com/bearcove/arborium/issues/101) | open | Implemented | Added Pug grammar, queries, sample, and upstream corpus. |
| [#98: Documentation doesn't appear to have a list of supported languages](https://github.com/bearcove/arborium/issues/98) | closed | Retained closed | Generated language tables and individual lang-* feature documentation retained and updated. |
| [#95: FR: add `diff`-like "hidden" grammar](https://github.com/bearcove/arborium/issues/95) | closed | Retained closed | Line hiding belongs in application preprocessing; upstream accepted that approach. |
| [#94: JS: `Span` uses UTF-8 offsets instead of UTF-16](https://github.com/bearcove/arborium/issues/94) | closed | Retained closed | Public JavaScript spans use UTF-16; documented DOM range usage. |
| [#93: JS: export mapping between capture and shorthand](https://github.com/bearcove/arborium/issues/93) | closed | Retained closed | Metadata export exists; #152 now adds the accurate renderer mapping helper. |
| [#92: JS/NPM: how to use grammar registry from the NPM packages?](https://github.com/bearcove/arborium/issues/92) | closed | Retained closed | Explicit resolver/registration model documented; importing a package alone has no registration side effect. |
| [#90: Box::leak usage in normalize_grammar](https://github.com/bearcove/arborium/issues/90) | closed | Retained closed | Store normalization uses Cow and owned map keys; no Box::leak normalization retained. |
| [#88: Selecting powershell crashes browser](https://github.com/bearcove/arborium/issues/88) | open | Unresolved: device-specific | All four named grammars initialize and parse in Node WASM checks. This does not reproduce an Android Chromium process crash; browser/device memory measurements are still required. |
| [#87: Improve READMEs for shared crates (miette-arborium, arborium-highlight, etc.)](https://github.com/bearcove/arborium/issues/87) | closed | Retained closed | Shared crate README generation exists; added a substantive Ratatui README source template. |
| [#78: miette-arborium should use the new ANSI API instead of parsing HTML](https://github.com/bearcove/arborium/issues/78) | closed | Retained closed | Historical miette integration refactor; no active miette crate in this checkout to change. |
| [#76: Highlighting fails if used in an existing wasm project](https://github.com/bearcove/arborium/issues/76) | closed | Retained closed | Superseded by #125; direct and all-language final WASM modules now tested. |
| [#75: NPM package: misnamed TS definition file (`dist/index.d.ts` instead of `dist/arborium.d.ts`)](https://github.com/bearcove/arborium/issues/75) | closed | Retained closed | Package types target arborium.d.ts; TypeScript package build/check remains the validation point. |
| [#74: Support WIT (WebAssembly Interface Types's IDL)](https://github.com/bearcove/arborium/issues/74) | closed | Retained closed | WIT grammar already bundled; included in all-grammar checks. |
| [#73: Jinja grammar seems Apache 2.0 licensed](https://github.com/bearcove/arborium/issues/73) | closed | Retained closed | Jinja metadata uses Apache-2.0; included in the permissive feature bundle. |
| [#72: How to add custom treesitter grammars locally?](https://github.com/bearcove/arborium/issues/72) | open | Already supported / documented | GrammarStore::insert supports local grammars. Expanded external-grammar docs to use LanguageFn and the Arborium runtime, avoiding duplicate native links. |
| [#71: ignoring invalid dependency `arborium` which is missing a lib target](https://github.com/bearcove/arborium/issues/71) | closed | Retained closed | Historical bad crates.io publish was yanked upstream; local/Git consumers have a committed lib target. |
| [#70: Support Apache Groovy syntax](https://github.com/bearcove/arborium/issues/70) | closed | Retained closed | Groovy grammar already bundled; included in all-grammar checks. |
| [#69: Q: Is there a CLI interface?](https://github.com/bearcove/arborium/issues/69) | closed | Retained closed | CLI exists; added locked source build and Nix packaging instructions. |
| [#67: Demo: no way to reset dark/light mode to system theme](https://github.com/bearcove/arborium/issues/67) | closed | Retained closed | Existing demo theme selection retained; documented removing data-theme for system preference. |
| [#66: Rendering requires js in dark theme?](https://github.com/bearcove/arborium/issues/66) | closed | Fixed closed-issue regression | Added system-dark fallback for static rustdoc pages without data-theme and fixed comma-separated selector scoping. |
| [#64: Make another pass at generated/gitignored/workspace'd files](https://github.com/bearcove/arborium/issues/64) | closed | Retained closed | Cargo manifests and language build inputs are now committed; generation remains a maintainer operation. |
| [#63: `/-` syntax miscolored in KDL v2](https://github.com/bearcove/arborium/issues/63) | closed | Retained closed | KDL query/grammar work was merged upstream; no new reproduction reported. |
| [#62: Support CSS Custom Highlight API output](https://github.com/bearcove/arborium/issues/62) | open | Documented extension | Added a DOM-preserving CSS Custom Highlight recipe using UTF-16 spans and getTagForCapture. A built-in renderer with injection, priority, and range lifecycle management remains separate work. |
| [#60: Import + add new tests](https://github.com/bearcove/arborium/issues/60) | open | Addressed | Added all-grammar query tests, issue reproductions, upstream corpora for changed/new grammars, a parallel corpus runner independent of generation, and parser test timeouts. |
| [#58: JS/TS: Shorthand object notation has no markup](https://github.com/bearcove/arborium/issues/58) | open | Fixed | Captured shorthand_property_identifier and shorthand_property_identifier_pattern as properties; JavaScript and TypeScript regressions. |
| [#57: KDL v2](https://github.com/bearcove/arborium/issues/57) | closed | Retained closed | KDL v2 work was merged upstream; no grammar replacement needed. |
| [#56: Dead link](https://github.com/bearcove/arborium/issues/56) | closed | Retained closed | Link failure was transient (lib.rs availability), according to the upstream resolution. |
| [#55: Pure Rust](https://github.com/bearcove/arborium/issues/55) | open | Not selected | A pure-Rust runtime and parser generator rewrite conflicts with the current C-parser architecture and required C compiler. It is not a maintenance fix. |
| [#53: Add README to arborium...](https://github.com/bearcove/arborium/issues/53) | closed | Retained closed | Umbrella README is generated and referenced in the manifest. |
| [#52: Now that we correctly add injection dependencies.. we must rejiggle publication order](https://github.com/bearcove/arborium/issues/52) | closed | Retained closed | Publish dependency ordering exists; new injection dependencies use the same dependency graph. |
| [#51: Runtime dependency on TOML should be eliminated](https://github.com/bearcove/arborium/issues/51) | closed | Retained closed | TOML theme parsing is optional; generated built-in theme data retained. |
| [#50: 'Generate' task DOESN'T NEED TO BUILD XTASK](https://github.com/bearcove/arborium/issues/50) | closed | Retained closed | CI reuses the generated xtask binary; corpus checks run independently of generation. |
| [#49: Add before/after RustDoc comparison demo with split panes](https://github.com/bearcove/arborium/issues/49) | closed | Retained closed | Static rustdoc comparison tooling exists; fixed its no-JavaScript theme case under #66. |
| [#48: Explore MDBook integration support](https://github.com/bearcove/arborium/issues/48) | closed | Retained closed | arborium-mdbook integration is present; no new implementation needed. |
| [#46: Theme color palette insufficient - only 7 colors available](https://github.com/bearcove/arborium/issues/46) | closed | Retained closed | Theme system already supports many slots and palettes; not limited to seven colors. |
| [#45: Public API review: error types, naming, and consistency cleanup](https://github.com/bearcove/arborium/issues/45) | closed | Retained closed | Existing Error/Highlighter/advanced API structure retained; added a separate Ratatui adapter without redesigning core APIs. |
| [#43: Grammar injection dependencies are incomplete](https://github.com/bearcove/arborium/issues/43) | closed | Retained closed | Fixed a remaining gap: umbrella lang-* features now activate their declared injection languages. |
| [#39: Make HTML tag mapping configurable](https://github.com/bearcove/arborium/issues/39) | closed | Retained closed | HtmlFormat supports custom elements and class names; retained renderer tests. |
| [#37: Document complete HTML tag mapping for syntax highlighting](https://github.com/bearcove/arborium/issues/37) | closed | Retained closed | Existing Rust mapping APIs retained; documented and exported the corresponding JavaScript helper. |
| [#36: Post-process docs.rs builds with html5ever for server-side syntax highlighting](https://github.com/bearcove/arborium/issues/36) | closed | Retained closed | arborium-rustdoc postprocessor exists; fixed scoped/no-script theme generation. |
| [#31: NPM dependencies are not precise enough](https://github.com/bearcove/arborium/issues/31) | closed | Retained closed | Existing generated package version and plugin manifest version retained. |
| [#30: Expose session API again](https://github.com/bearcove/arborium/issues/30) | closed | Retained closed | Session API exists; documentation emphasizes UTF-16 offsets and free() lifecycle. |
| [#29: CI publishes crates blindly without checking if content hash matches](https://github.com/bearcove/arborium/issues/29) | closed | Retained closed | Publish hashing code exists; retained content-hash generation for regenerated grammar inputs. |
| [#27: YAML serializer should use block scalar syntax for multi-line strings](https://github.com/bearcove/arborium/issues/27) | closed | Retained closed | Upstream marked this as belonging to the Facet repository; no Arborium change selected. |
| [#26: Add ANSI escape output format support](https://github.com/bearcove/arborium/issues/26) | closed | Retained closed | AnsiHighlighter and ANSI rendering are present; #170 is the inverse conversion. |
| [#22: Consolidate langs/ crates into a workspace to reduce compilation time](https://github.com/bearcove/arborium/issues/22) | closed | Retained closed | Shared Cargo target directories are used for tests; no workspace redesign needed for Cargo Git dependency support. |
| [#20: In iife docs.rs themes + theme selector / light-dark mode selection](https://github.com/bearcove/arborium/issues/20) | closed | Retained closed | Rustdoc light/dark/ayu theme integration exists; static dark-mode fallback now fixed. |
| [#18: Main package version isn't following CI tag version](https://github.com/bearcove/arborium/issues/18) | closed | Retained closed | Canonical version is propagated by generation; no version bump or package publication performed. |
| [#17: Visual regression testing with screenshot comparison](https://github.com/bearcove/arborium/issues/17) | closed | Retained closed | Visual regression infrastructure is a separate historical request; this review adds semantic/corpus checks for reproducible failures. |
| [#16: Use shared WASI preview2-shim instead of per-plugin instantiation mode](https://github.com/bearcove/arborium/issues/16) | closed | Retained closed | Upstream rejected this WASI shim approach; current wasm-bindgen architecture retained. |
| [#15: Revamp publish flow](https://github.com/bearcove/arborium/issues/15) | closed | Retained closed | Existing pre/grammar/post publishing workflow retained; Ratatui added to post group. |
| [#14: Improve generated crate metadata for docs.rs and crates.io](https://github.com/bearcove/arborium/issues/14) | closed | Retained closed | Generated crate metadata/README templates already supply descriptions, license, links, and usage. |
| [#11: npm distribution: @arborium packages](https://github.com/bearcove/arborium/issues/11) | closed | Retained closed | npm plugin and IIFE distribution architecture already exists; offline integration docs expanded. |
| [#8: Replace tree-sitter-highlight with custom highlighting implementation](https://github.com/bearcove/arborium/issues/8) | closed | Retained closed | Custom Arborium highlighting/rendering implementation is present; reused its flat-token logic for Ratatui. |
| [#6: Speed up Windows CI with dev drive](https://github.com/bearcove/arborium/issues/6) | closed | Retained closed | Windows CI configuration retained; no unsupported performance changes introduced. |
| [#5: Remove wasm-fix feature flag and optimize WASM CI job](https://github.com/bearcove/arborium/issues/5) | closed | Retained closed | WASM behavior is selected through target cfg; no wasm-fix feature reintroduced. |
| [#4: Document miette-arborium and showcase in HTML demo](https://github.com/bearcove/arborium/issues/4) | closed | Retained closed | Historical miette publishing/documentation request; no active crate in this checkout to modify. |
| [#1: Plugin design](https://github.com/bearcove/arborium/issues/1) | closed | Retained closed | Original plugin design was superseded by the current wasm-bindgen implementation; no ABI redesign selected. |

## Additional findings

- Solidity's query contained invalid capture/anchor syntax; fixed and covered by
  the generated all-grammar query test.
- [PR #211](https://github.com/bearcove/arborium/pull/211) identified a COBOL
  external-scanner loop at EOF. The scanner now stops at EOF, with native/WASM regressions.
- The test harness retained a static-lifetime assumption invalidated by tree-sitter
  0.27. It now ties collected node-kind strings to the tree lifetime.
- The WASM sysroot's `static_assert` expansion was an expression, invalid at file
  scope. It now uses C's compile-time assertion, discovered by the Crystal build.

## Validation

Checks run on macOS with Rust 1.98.1 and Homebrew LLVM for WASM:

- `cargo test --manifest-path crates/arborium/Cargo.toml --all-features`:
  176 passing tests, including query compilation for all 119 grammars and 16
  issue reproductions; eight pre-existing documentation examples remain ignored.
- The Markdown regression also passes with only `--features lang-markdown`.
  Cargo metadata confirms that `all-permissive-languages` excludes Nginx and Uiua while
  resolving the new grammars and Markdown's internal injection grammar.
- `python3 scripts/check_corpus.py`: all nine upstream corpus suites pass.
  SCSS and Slang expected trees account for the newer bundled CSS/C++/HLSL
  dependencies; their input fixtures are unchanged.
- `python3 scripts/check_wasm.py`: direct runtime and all 119 grammars link and
  execute in Node with zero host imports. The reported F#, Perl, COBOL, Markdown,
  PowerShell, Swift, Vim, and Caddy cases complete within the timeout.
- `python3 scripts/check_git_dependency.py --wasm`: a separate consumer builds
  using 18 Arborium packages from a temporary Git snapshot, without generation,
  and passes the native execution and WASM link checks.
- Ratatui: three tests pass. Rustdoc: nine tests pass. Just: both tests pass with
  `CFLAGS=-DNDEBUG`. xtask: 19 tests pass.
- The compiler probe accepts Homebrew LLVM and this machine's Apple clang 21
  (which now includes a WASM backend). A deliberately native-target compiler
  configuration is rejected with the LLVM setup diagnostic.
- JavaScript: six tests pass, including renderer mapping aliases; TypeScript
  typechecking passes.
- The standalone CLI builds with its committed lockfile. Generated CI matches
  `cargo xtask ci generate --check`.

Nix is not installed here, so the new package expression has not been executed.
The Android Chromium crash in #88 requires device/browser testing; Node WASM
success is not evidence that this browser-process crash is fixed. The complete
wasm-bindgen/wasm-opt plugin packaging pipeline was not run locally; its generated
CI jobs include the new grammars.
