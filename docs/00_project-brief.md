---
title: "Project Brief"
description: "Purpose, scope, acceptance criteria, and high-level design."
keywords: [brief, overview, design, scope, acceptance-criteria]
order: 0
---

# Project Brief

## Why this exists

`gol` is Conway's Game of Life running as a Bevy app. The simulation lives
in a desktop window so you can watch cells live and die under the classic
B3/S23 rules.

## What the tool does

- Opens a Bevy window on launch
- Builds a cell grid from the window size during startup (entities are not
  recreated each tick; the window is a viewport onto that canvas)
- Seeds that grid with a Gosper glider gun plus a few small patterns
- Steps the grid on a repeating timer (8 ticks per second)
- Paints live, shielded, and dead cells by updating sprite colors in place
- Left-mouse hold draws live cells with a few ticks of death shield
- Right-mouse hold erases cells
- Space pauses and resumes the simulation so a shape can be drawn first
- R resets the board to the classic seed

## Run

```bash
cargo run
# or
cargo make r
```

There are no CLI flags. Close the window to quit.

## Acceptance criteria

1. `cargo run` opens a Bevy window (not stdout-only).
2. Startup spawns one sprite per cell on an 80x60 grid.
3. The grid size does not change after init.
4. Cells follow B3/S23: birth on 3 neighbors, survival on 2 or 3, death
   otherwise. Edges wrap (torus).
5. Unit tests cover still lifes, oscillators, a glider, wrapping, and
   bounds errors.
6. Holding the left mouse button paints a stroke of shielded cells.
   Shielded cells ignore underpopulation until the shield expires, then
   B3/S23 applies.
7. Space toggles pause. Drawing still works while paused; unpausing
   resumes the timer so the drawn shape evolves.
8. Holding the right mouse button erases a stroke of cells to dead.
9. R clears the board and restamps the classic seed. Pause state is kept.

## Out of scope

- Speed controls
- Infinite or dynamically resized grids
- Saving or loading patterns from disk
- Hex or other non-square neighborhoods
