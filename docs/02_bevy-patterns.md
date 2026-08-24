---
title: "Bevy Patterns"
description: "Established Bevy 0.19 patterns and pitfalls learned while building gol."
keywords: [bevy, patterns, testing, messages, states, time, headless, pitfalls]
order: 2
---

# Bevy Patterns

Practices established in this codebase, drawn from the bevy_game_template
structure and the Bevy 0.19 docs. Follow them for new systems; do not
regress them when refactoring.

## Plugin structure

- One plugin per domain; each domain is a folder with `mod.rs` holding the
  plugin struct, resource init, and system registration. The template repo
  (NiklasEi/bevy_game_template) uses this layout: gameplay / input / render
  style splits keep scheduling explicit and tests local.
- Register everything a domain's systems need inside that domain's
  `Plugin::build`. A system whose resource was never initialized compiles
  fine and panics on frame one with "Resource does not exist" -- and unit
  tests that manually insert resources will not catch it. Always boot the
  app once after wiring changes.
- Cross-domain consumption (two domains touching one resource) means the
  resource belongs where it is primarily mutated; root-level glue systems
  handle the coordination.

## System sets

Group a domain's systems into named `SystemSet`s (e.g. `InputSet::Keyboard`,
`InputSet::Pointer`) at registration:

```rust
.add_systems(
    PreUpdate,
    (
        handle_key_actions.in_set(InputSet::Keyboard),
        handle_pointer_actions.in_set(InputSet::Pointer),
    ).chain(),
);
```

Other domains order against the set, not individual systems, so renaming or
adding handlers never ripples outward.

## States

- Derive `States` on a small enum; gate systems with `.run_if(in_state(...))`
  at registration.
- Transitions are deferred: writing `NextState<S>` applies during the next
  frame's `StateTransition` schedule. Any test that flips state must run two
  `app.update()` frames after the trigger before asserting.

## Messages

- Bevy 0.19 renamed events to messages (`MessageReader` / `MessageWriter`,
  `add_message::<T>`, `write_message`). Buffered two frames by default.
- A reader added after the write can still observe this frame's writes via
  `Messages::<T>::iter_current_update_messages`.
- Use messages for one-to-many notifications (`ResetBoard`, `AppExit`);
  use resources for shared mutable state.

## Newtypes over math types

Components that wrap a single value (positions, focuses) are tuple structs
deriving `Deref`, with `From` conversions to/from the underlying math type:

```rust
#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct CellPos(pub Vec2);
```

Call sites use the conversions instead of field access, so the wrapper can
change shape without touching callers. The same rule applies to resources
(`ViewportFocus(pub Vec2)`).

## Time and timers

`MinimalPlugins` runs time-advance systems every frame that overwrite any
manually advanced virtual time. Timer-gated logic therefore cannot be tested
through `App::update` in a headless test. Drive the `Timer` directly:

```rust
let fired = timer.tick(Duration::from_secs_f32(TICK)).just_finished();
```

and keep the system body thin enough that this seam is honest.

## Headless testing limits

- Synthetic window entities carry no real `Window` component, so cursor
  picking (`window.cursor_position()`, camera viewport projection) cannot be
  exercised headless. Test the pure geometry helpers (`world_to_cell`) on
  their own instead of through fake window messages.
- Keyboard and state-machine flows DO work headless: register the message
  types, init the input resource, write a synthetic `KeyboardInput`, update,
  assert.
