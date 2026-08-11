# rust-wasm-pages-template

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template
for client-side tools: a pure Rust core compiled to WASM, a framework-free
web page, deployed as static files to GitHub Pages, working offline via a
service worker. The user's file never leaves their machine.

Proven shape — extracted from
[alters-save](https://github.com/Iteratrix/alters-save) (a game save editor)
and propwash (a drone blackbox analyzer), which share it end to end.

## Generate a project

```sh
cargo install cargo-generate
cargo generate Iteratrix/rust-wasm-pages-template
```

You'll be asked for a project name and whether to include a CLI crate
(recommended: it's nearly free and doubles as a test harness).

## What you get

```
{{project-name}}-core/   pure domain logic — no I/O, wasm-safe deps only
{{project-name}}-web/    thin wasm-bindgen bridge (JSON-string API)
{{project-name}}-cli/    thin CLI adapter (optional)
web/                     framework-free page + build.mjs + service worker
.github/workflows/       CI (clippy/fmt/test/wasm) + Pages deploy on tag
.claude/skills/          the pattern's design docs, readable by humans and
                         picked up automatically by Claude Code
```

The generated project builds, tests, and deploys as-is: drop a file on the
page and a placeholder `summarize` function describes it. Replace core's
logic with your own and grow outward.

## Dev loop

```sh
cargo test
wasm-pack build {{project-name}}-web --target web --out-dir ../web/pkg
python3 -m http.server -d web   # or any static server
```

The service worker is skipped on localhost, so dev never fights a stale
cache.

## Deploy

One-time: repo Settings → Pages → Source: "GitHub Actions". Then:

```sh
git tag v0.1.0 && git push origin v0.1.0
```

`web/build.mjs` (zero npm dependencies) content-hashes every asset, rewrites
references, and generates a versioned service worker — precached shell,
network-first navigations, cache-first hashed assets, no `skipWaiting`. If a
broken worker ever bricks cached clients, deploy `web/sw-killswitch.js` as
`sw.js` to recover them.

The full design rationale and invariants live in
[.claude/skills/wasm-pages-app/SKILL.md](.claude/skills/wasm-pages-app/SKILL.md) —
written as plain design docs; Claude Code additionally picks them up as a
skill in generated projects.

## After generating

- Replace the placeholder `Summary`/`summarize` in core with your logic.
- Update `LICENSE` with your name.
- Vendor any third-party JS under `web/vendor/` and add it to `build.mjs` —
  don't use CDNs; they break offline support.
