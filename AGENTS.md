# AGENTS.md (agent-only -- not committed)

This file is local agent guidance. It is listed in `.git/info/exclude` and must never
be committed. Do not reference it from published docs.

## Project

`gol` is Conway's Game of Life in a Bevy window. The board is a fixed
canvas sized from the initial window. The window is a camera viewport
centered on `ViewportFocus` (world origin). Resize does not delete cells.
The crate is split by domain plugin (`gameplay`, `input`, `render`),
mirroring the bevy_game_template layout; see `docs/01_architecture.md`.

## Doc access (mandatory -- prevents context overloading)

NEVER ingest the entire `docs/` directory. The docs folder can grow large
and dumping it all into context wastes tokens and muddies focus.

1. ALWAYS read `docs/index.md` first -- it is the searchable index of all
   documentation, with titles, descriptions, and keywords for each doc.
2. Use the index to identify which specific doc(s) your task needs.
3. Load ONLY those specific docs. One or two at most per task.
4. Do not read docs "just in case." If you don't know which doc you need,
   the index keywords will point you there.

When adding a new doc to `docs/`:
1. Create `docs/NN_topic-name.md` with YAML frontmatter (title, description,
   keywords, order). See `docs/00_project-brief.md` for the format.
2. Run `python3 scripts/update-docs-index.py` to regenerate the index.
   This makes the new doc discoverable via the index without manual editing.

Do not adjust `scripts/update-docs-index.py`! Prefer updating existing docs
to be COMPLIANT with the script's frontmatter format. If the required changes
to existing docs are large or complex beyond simple conformance: STOP and
consult the user. Detail what the current state IS, what needs to occur to
make docs compliant, and why the changes are deemed large or complex.

Docs are written for all readers, not for agents specifically. Do not
address "the agent" inside `docs/`; phrase guidance toward humans.

## Hard rules

- VCS: this is a `jj` (colocated git) repo. Use `jj` only. Never run raw `git` here;
  it corrupts the jj graph. Do not commit or push unless asked.
- Text: ASCII only in all project files. No em-dash (use `--`), no unicode arrows
  (use `->`), no box-drawing. Comments use plain `// Section`, not decorated
  separators.
- Don't delete work files. Archive superseded drafts to `_archive/` instead of `rm`.
- Execute defined tasks directly. Don't re-ask for confirmation on low-stakes choices;
  state the decision in your summary.
- Root cause over quick patches. When you fix something, check sibling paths for the
  same flaw.

## Testing (hard rule)

Verification is `cargo build` + `cargo test` + running the app. Nothing else.

- Use the crate's own Rust test suite (`cargo test`). Do NOT write standalone
  verification scripts, Python harnesses, or ad-hoc shell checks to prove a
  change works.
- Do NOT use `hermes verify`'s HTTP readiness poll for this project: it is a
  desktop app with no server port, so that phase always fails meaninglessly.
  Use `--skip-start`, or just `cargo test` plus launching the app once.
- Window spawning is verified by the human looking at the screen, not by tooling.
- Headless App-based tests are acceptable only when they exercise real system
  logic (state machines, message flow). Never fight `MinimalPlugins` time
  systems: test timers/stepping logic directly instead of through `App::update`.
- Tests live inline in `#[cfg(test)] mod tests` per module. `use super::*;`
  is allowed there -- and ONLY there.

## Build and run

- `cargo run -- <args>` or `cargo make r` to run.
- `cargo make A` is the full verify sweep (format, check, clippy, build, test).
- Release binaries: `build/release-tasks.toml` + `build/install.sh`.

## Source layout

Domain-plugin structure (bevy_game_template style):

- Entry point: `src/main.rs` (`App::run` lives here, not in the lib)
- Library map + root `GolPlugin` + `SimState`: `src/lib.rs`
- Gameplay domain (`GameplayPlugin`: grid, cell rules, sim tick, seed):
  `src/gameplay/{grid,cell,sim,seed}.rs`
- Input domain (`InputPlugin`: bindings, brush, key/mouse handlers,
  `InputSet::{Keyboard,Pointer}`): `src/input/{action,bindings,handle_key,handle_mouse}.rs`
- Render domain (`RenderPlugin`: camera, `ViewportFocus`, board sprites,
  palette): `src/render/{camera,board_view,palette}.rs`
- Error types: `src/error.rs`
- Human-facing docs: `docs/` (index maintained by `scripts/update-docs-index.py`)
- Build tasks: `build/common-tasks.toml`, `build/release-tasks.toml`
- CI: `.github/workflows/` (build, test, format, docs, draft, publish)

## Hard-won patterns (do not regress)

Rust:

- NEVER write `fn as_str(&self) -> &str` or `fn into_string(self) -> String` on a
  string newtype. Use `impl Deref<Target = str>` + `From<String>` / `From<&str>`.
- Conversions go through std traits: implement/derive `From`/`TryFrom` (+ `Deref`
  where a plain inner value makes sense) instead of bespoke `from_<foo>()` /
  `to_<bar>()` methods. Call sites then read `T::from(x)` / `x.into()` and work
  with `?`, iterators, and generics for free.
- Prefer `thiserror #[from]` over manual `map_err` when the source error implements
  `std::error::Error`.
- Resolve `Option` at the boundary (CLI parse, path discovery) before entering the
  pipeline. No `Option<T>` fields in core types.
- Use `fn read<P: AsRef<Path>>(path: P)` NOT `fn read(path: impl AsRef<Path>)`.

Imports:

- No `super::` paths outside test modules. Sibling modules import via
  `crate::<domain>::<Item>` (re-exported at each `mod.rs`) or direct private-module
  paths within their own domain. Keeps refactors flat.

Bevy 0.19 specifics learned during the refactor:

- Resources used by any registered system MUST be initialized in the owning
  plugin's `build` (`init_resource`), including cross-domain ones consumed by
  root glue (e.g. `Brush` lives in input because both input systems and the
  root reset handler touch it). Missing init compiles fine and panics on first
  frame -- tests that insert resources manually will NOT catch it. Boot the app
  once after plugin wiring changes.
- `States` transitions are deferred: writing `NextState` takes effect on the
  NEXT frame's `StateTransition` schedule. Tests asserting state flips need two
  `app.update()` calls after the triggering input.
- `MessageReader` buffers two frames; `iter_current_update_messages` sees
  messages written this frame even if the reader was added after the write.
- `MinimalPlugins` runs real-time systems that overwrite manually advanced
  virtual time every frame -- do not test timer-gated systems through
  `App::update`; drive the `Timer` directly.
- Synthetic window entities in headless tests have no real `Window` component:
  pointer-picking tests cannot work headless. Test the pure geometry helpers
  instead.
- Group related systems into named `SystemSet`s per domain (`InputSet`) so
  other domains can order against them without knowing internal system names.
