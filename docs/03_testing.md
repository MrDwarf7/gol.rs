---
title: "Testing"
description: "How this project is verified: Rust test suite conventions and limits."
keywords: [testing, verification, cargo-test, headless, unit-tests]
order: 3
---

# Testing

The only accepted verification is the crate's own test suite plus launching
the app. No standalone verification scripts, no ad-hoc harnesses.

## Commands

```bash
cargo test      # unit tests, inline per module
cargo make A    # full sweep: format, check, clippy, build, test
```

Window spawning is confirmed by a human looking at the screen; there is no
automated readiness check for a desktop app.

## Conventions

- Tests live in `#[cfg(test)] mod tests` at the bottom of each module.
  `use super::*;` is expected there -- it is the one sanctioned use of
  `super::`.
- Test pure logic directly (grid stepping, timer firing, chord lookup)
  rather than through `App` scaffolding where possible.
- Headless `App` tests are reserved for real ECS behavior: state
  transitions (two updates needed), message flow, system registration.

## Known limits

- Pointer picking cannot be tested headless: synthetic window entities have
  no real `Window`. Cover the geometry helpers instead.
- `MinimalPlugins` overwrites manually advanced virtual time each frame;
  drive timers directly rather than through `App::update`.
- Tests that insert resources manually hide missing `init_resource` calls.
  After changing plugin wiring, boot the app once.
