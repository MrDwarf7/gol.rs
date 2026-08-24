---
title: "Architecture"
description: "Domain-plugin layout, plugin wiring, state machine, and startup order."
keywords: [architecture, plugins, domains, states, simstate, wiring, startup, systems]
order: 1
---

# Architecture

The crate is organized as three domain plugins under a root plugin,
mirroring the bevy_game_template structure. Each domain owns its
resources, systems, and tests; cross-domain types are re-exported
through each `mod.rs` so callers never reach into sibling internals.

## Domains

- `gameplay` (`GameplayPlugin`): the simulation. Owns the `Grid`
  resource (built once in `PreStartup` from the initial window), cell
  rules (`CellState`, B3/S23 with shielded cells and toroidal wrap),
  the timer-gated `advance_generation` step system, and seeding.
- `input` (`InputPlugin`): converts raw keyboard/pointer messages into
  actions. Owns key bindings (`Bindings`, `Chord`), pointer bindings
  (`PointerBindings`), the `Brush` stroke state, and the two handler
  systems grouped into `InputSet::Keyboard` / `InputSet::Pointer`.
- `render` (`RenderPlugin`): everything drawn. Owns the camera setup
  and `ViewportFocus`, one sprite per grid slot (`CellPos` component),
  and the palette-based repaint pass.

## Root glue

`GolPlugin` in `src/lib.rs` owns the shared `SimState` machine
(`Running` / `Paused`) and the cross-domain reset path: a keyboard
action writes an `ResetBoard` message; a root `PreUpdate` system reads
it, clears the input brush, and reseeds the gameplay grid. Cross-domain
glue belongs at the root, not inside either domain.

## Startup order

1. `PreStartup`: `GameplayPlugin` sizes the fixed `Grid` from the
   initial window resolution.
2. `Startup`: seed the classic pattern, spawn camera, spawn one sprite
   per grid slot.
3. `Update`: step the simulation on a 0.125 s repeating tick, but only
   while `SimState::Running` (`run_if(in_state(...))`). Input handlers
   run earlier in `PreUpdate`.
4. `PostUpdate`: repaint sprite colors from grid state unconditionally,
   so a paused reset still redraws the board.

## State machine

Pause/resume is a real Bevy state (`States` derive on `SimState`).
Input writes `NextState<SimState>`; the transition applies on the next
frame's `StateTransition` schedule. Systems gate on
`in_state(SimState::Running)` at registration time.

## Conventions

- Resources used by a system must be initialized by the owning plugin
  (`init_resource`). A missing init compiles cleanly and panics on the
  first frame; boot the app after changing plugin wiring.
- New system groups get a named `SystemSet` in their domain so other
  domains can order against it without knowing internal names.
- Position-like components are tuple newtypes over their math type
  with `Deref` plus `From` conversions (see `CellPos(pub Vec2)`).
