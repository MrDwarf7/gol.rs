---
title: "TODO: Vulkan Present Validation Errors At Startup"
description: "Why gol logs VUID-VkPresentInfoKHR-pImageIndices-01430 on the first frames, and the state-gated camera spawn that fixes it."
keywords: [vulkan, validation, wgpu, present, swapchain, camera, startup, pipeline-cache, state]
order: 91
---

# TODO: Vulkan Present Validation Errors At Startup

Status: diagnosed, fix designed, NOT yet implemented.

Running `gol` logs two Vulkan validation errors during startup:

```text
ERROR wgpu_hal::vulkan::instance: VALIDATION [VUID-VkPresentInfoKHR-pImageIndices-01430]
  vkQueuePresentKHR(): pPresentInfo images passed to present must be in layout
  VK_IMAGE_LAYOUT_PRESENT_SRC_KHR or VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR
  but VkImage 0x780000000078 is in VK_IMAGE_LAYOUT_UNDEFINED.
```

They are harmless on this machine (NVIDIA RTX 4080, driver 610.57.04): they
appear only on the first two presented frames and never again. On some Mesa /
Intel setups the same condition is reported alongside visible flicker, so it is
worth removing rather than ignoring.

## Reproduction

- Ours: `cargo run`, then count `VUID` lines. Result: 2 validation errors
  (4 log lines including the `objects:` detail lines), both within ~1.4 s of
  launch, then a clean log for the remaining runtime.
- Template: `cargo run` in a checkout of the upstream template. Result: 0
  validation errors on the same GPU and driver.

Note when reproducing the template: run it with `cargo run`, not by invoking
`target/debug/bevy_game` directly. Bevy resolves the asset folder relative to
the executable, so the bare binary fails to load its assets, never leaves
`GameState::Loading`, and therefore never spawns a camera. A cameraless run
logs no present errors for the wrong reason and invalidates the comparison.

## Root cause

The trigger is a Bevy/wgpu interaction, not project logic. The chain:

1. `bevy_render` acquires a swapchain image every frame in `prepare_windows`
   and, for the first frame, presents unconditionally: in
   `bevy_render/src/renderer/mod.rs` the guard is
   `if view_needs_present || window.needs_initial_present`, and
   `needs_initial_present` is initialized to `true`. This exists because
   Wayland requires an initial present.
2. The blit/upscaling pipeline that would write to that swapchain image is
   compiled asynchronously by `PipelineCache`. On the first frames
   `pipeline_cache.get_render_pipeline(...)` returns `None`.
3. `bevy_core_pipeline/src/upscaling/node.rs` then returns early. The early
   return only opens a render pass under `#[cfg(any(target_os = "macos",
   target_os = "ios"))]`, so on Linux/Vulkan no pass is recorded and no layout
   transition happens.
4. The image is presented while still in `VK_IMAGE_LAYOUT_UNDEFINED`. wgpu only
   transitions a surface texture to `PRESENT` when it was used in a submit, so
   nothing corrects it. Validation fires.

Upstream tracking: bevyengine/bevy#22733 and gfx-rs/wgpu#9213 (fix PR
gfx-rs/wgpu#9222 addresses presenting after no work was done).

## Why the template does not hit it

Not a render-settings difference. The template spawns its cameras from
`OnEnter(GameState::Menu)` and `OnEnter(GameState::Playing)`, and it only
reaches `Menu` after `bevy_asset_loader` finishes loading its asset
collections. By the time a camera exists, the pipeline cache has compiled the
upscaling pipeline, so every presented frame has a real render pass behind it.

`gol` spawns its camera in `Startup`, so a camera is a render target on frame
zero, before the pipeline is ready. Same engine, different timing.

## Ruled out

- `Msaa::Off` on the camera (the template sets it; we do not). Tested: it makes
  the problem worse, not better -- 4 present errors instead of 2, plus a new
  `VUID-vkAcquireNextImageKHR-semaphore-01286`. Reverted. `Msaa` is unrelated to
  the layout transition and should not be added as a fix.
- `bevy_core_pipeline` version skew. Ours resolves 0.19.1, the template 0.19.0,
  but the only delta in `upscaling/node.rs` between them widens the macOS
  `cfg` to include iOS. Linux behaviour is identical in both.
- Grid initialization order. The grid is sized in `PreStartup` and is not part
  of the present path.

## Required fix

Gate camera spawn on a state transition rather than `Startup`, following the
template's pattern. A `MainMenu` state is planned anyway, which gives the
natural seam:

1. Add a startup/menu variant to the root state machine in `src/lib.rs` (e.g.
   extend `SimState`, or add a separate `AppState` with `Loading` / `Menu` /
   `InGame`).
2. Move `spawn_camera` off `Startup` and onto `OnEnter(<in-game state>)` in
   `RenderPlugin`, so the first frames present with no camera target.
3. Keep `spawn_cells` where it is; sprites without a camera are harmless.

Verification of the mechanism: temporarily gating `spawn_camera` behind a
`FrameCount` check (spawn only after frame 10) produced a run with zero
validation errors and a correctly rendering board. That confirms spawn timing
is the whole story. The frame-count gate was a probe only and has been
reverted -- do not ship it; use the state transition.

Alternative if the state split is not wanted: set
`synchronous_pipeline_compilation: true` in `RenderPlugin`'s `WgpuSettings`,
which forces the pipeline to exist before the first present at the cost of a
slower startup. Prefer the state gate.

## Reference

Upstream template this project's layout mirrors:
<https://github.com/NiklasEi/bevy_game_template>

Useful for future comparisons of plugin structure, state-driven setup, CI, and
release packaging. When comparing runtime behaviour against it, remember the
`cargo run` caveat above.
