---
title: Development
---

# Development

[Home](index.md) · [Usage](usage.md) · [Parsing](parsing.md) · [Settings](settings.md)

## Build and test

```sh
cargo run            # debug build
cargo run --release  # optimized build
cargo test           # parser and settings unit tests
cargo fmt            # format
```

The first build compiles GPUI and is slow; later builds are fast.

## Project layout

| File              | Purpose                                                      |
| ----------------- | ------------------------------------------------------------ |
| `src/main.rs`     | The UI: window, environment cards, table, settings panel     |
| `src/parser.rs`   | `.env` parsing, row comparison and character diff (no UI)    |
| `src/settings.rs` | Load and save preferences as JSON                            |

Logic that doesn't need a window is kept in `parser.rs` and `settings.rs` so it
can be unit tested without a GPU.

## Dependencies

- [`gpui-kit`](https://crates.io/crates/gpui-kit): one crate that bundles GPUI
  and the styled component library (inputs, table, select, checkbox, buttons).
- `serde`, `serde_json` and `dirs` for settings.

`gpui-kit` pins a snapshot of GPUI published as `gpui-pre`, not the `gpui` crate
on crates.io. When reading GPUI documentation, expect some API differences. The
crate sources under `~/.cargo/registry/src` are the most reliable reference.

## Notes

- The icon set is a separate asset bundle. The app passes it with
  `.with_assets(gpui_kit::assets::Assets)`; without it, icons such as checkbox
  ticks do not render.
- On macOS GPUI may need the Metal toolchain from a full Xcode install. It built
  here without one, but if you hit shader or GPU errors, install Xcode and run
  `sudo xcode-select -s /Applications/Xcode.app`.

## Publishing these docs

The docs are plain Markdown in `docs/`, rendered by GitHub Pages with Jekyll.
In the repository settings, open **Pages** and set the source to the `main`
branch and the `/docs` folder.
