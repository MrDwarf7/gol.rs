# TODO

Open work items. Each entry links to a detailed doc under `docs/`.

- [ ] **Vulkan present validation errors at startup** --
  `gol` logs two `VUID-VkPresentInfoKHR-pImageIndices-01430` errors on the
  first frames because the camera spawns in `Startup`, before the upscaling
  pipeline finishes compiling, so the initial swapchain present has no render
  pass behind it. Fix is to spawn the camera on a state transition (the
  planned main menu) instead of `Startup`. Diagnosed and verified; not yet
  implemented. See
  [docs/TODO_01_vulkan-present-validation.md](docs/TODO_01_vulkan-present-validation.md).
