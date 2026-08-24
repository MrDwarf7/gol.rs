# gol

Conway's Game of Life in a Bevy window. A fixed cell canvas is built from
the initial window size and stepped with B3/S23 rules on a 0.125 s tick;
the window is a camera viewport onto that canvas, so resizing never deletes
cells.

## Controls

| Input | Action |
|-------|--------|
| Left click / drag | Draw living cells |
| Right click / drag | Erase cells |
| Space | Pause or resume the simulation |
| R | Reset to the starter scene |
| Q or Escape | Quit |

The board starts seeded with a Gosper glider gun plus a glider, blinker,
toad, and beacon. Cells wrap around the edges (toroidal).

## Build

Requires a recent nightly Rust toolchain (see `rust-toolchain.toml`).
Linux needs ALSA, udev, and X11 dev packages:

```bash
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev \
  libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
```

```bash
cargo make r    # run it
cargo make A    # format + check + clippy + build + test sweep
cargo test      # tests only
```

Dev builds use Bevy's dynamic linking through the `dev` feature, which
keeps iteration fast; release builds are fully static.

## Install

### From source

```bash
cargo install --git https://github.com/mrdwarf7/gol.rs
```

### Release archives

Download from [Releases](https://github.com/mrdwarf7/gol.rs/releases/latest).

| OS | Arch | Triple |
|----|------|--------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |

## Layout

Domain-plugin structure following bevy_game_template:

- `src/gameplay/` -- grid resource, B3/S23 rules, sim tick, pattern seeding
- `src/input/` -- key bindings, mouse brush, pause state transitions
- `src/render/` -- camera, viewport focus, board sprites, palette

Further reading lives in [docs/index.md](docs/index.md).

## License

MIT -- see [LICENSE](LICENSE).
