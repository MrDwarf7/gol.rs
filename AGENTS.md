# AGENTS.md (agent-only -- not committed)

This file is local agent guidance. It is listed in `.git/info/exclude` and must never
be committed. Do not reference it from published docs.

## Project

`gol` is a Rust CLI project scaffolded from `rust_template`. Edit this
section with a one-paragraph description after running `setup.sh`.

## First-run onboarding

If this project has no commits yet (bare/scaffolded state) or
`docs/00_project-brief.md` still contains template placeholder text,
read `docs/00_AGENT_DOCS_SETUP.md` immediately. It guides a conversation
with the user to populate project docs before any code work begins.

After the onboarding is complete and all docs are filled in, delete both
`docs/00_AGENT_DOCS_SETUP.md` and this "First-run onboarding" section
from AGENTS.md. Neither should persist once the project is established.

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

## Build and run

- `cargo run -- <args>` or `cargo make r` to run.
- `cargo make A` is the full verify sweep (format, check, clippy, build, test).
- Release binaries: `build/release-tasks.toml` + `build/install.sh`.

## Source layout

- Entry point: `src/main.rs`
- Error types: `src/error.rs`
- Prelude: `src/prelude.rs`
- Human-facing docs: `docs/` (index maintained by `scripts/update-docs-index.py`)
- Build tasks: `build/common-tasks.toml`, `build/release-tasks.toml`
- CI: `.github/workflows/` (build, test, format, docs, draft, publish)

## Hard-won patterns (do not regress)

- NEVER write `fn as_str(&self) -> &str` or `fn into_string(self) -> String` on a
  string newtype. Use `impl Deref<Target = str>` + `From<String>` / `From<&str>`.
- Prefer `thiserror #[from]` over manual `map_err` when the source error implements
  `std::error::Error`.
- Resolve `Option` at the boundary (CLI parse, path discovery) before entering the
  pipeline. No `Option<T>` fields in core types.
- Use `fn read<P: AsRef<Path>>(path: P)` NOT `fn read(path: impl AsRef<Path>)`.
