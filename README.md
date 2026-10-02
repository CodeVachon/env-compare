# ENV Compare

A small desktop app for comparing `.env` files side by side. Paste the contents
of two or more env files (or open the files themselves) and get a table with one
row per key and one column per environment.

|           | Env1                   | Env2                   |
| --------- | ---------------------- | ---------------------- |
| `API_KEY` | `djfjfkdkd`            | `sldkfjslkd`           |
| `SECRET`  | `dfgdfgdf`             | `dfgfgdf`              |
| `URL`     | `https://api.env1.com` | `https://api.env2.com` |

Built with Rust, [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui)
(the UI framework behind Zed) and [GPUI Kit](https://gpui-kit.com/).

## Features

- **Paste or load** any number of environments: paste into a text area, use
  **Open files…**, or drag files onto the window.
- **Show only differences** hides every key whose value is identical everywhere.
  A key missing from one environment counts as a difference.
- **Highlight differences** marks the individual characters that don't match, so
  `0` vs `O` or a single wrong letter in an API key stands out.
- **Clear** empties one environment without removing its column.
- **Dark / light mode** and a **custom font** picked from your installed fonts.
  Both are remembered between launches.
- Monospace font by default.

## Install and run

You need a recent stable [Rust toolchain](https://rustup.rs/).

```sh
git clone https://github.com/CodeVachon/env-compare.git
cd env-compare
cargo run --release
```

It has been built and run on macOS. GPUI also targets Linux and Windows, but
those platforms are untested here.

## Documentation

Full docs live in [`docs/`](docs/index.md) and are published with GitHub Pages:

- [Usage](docs/usage.md)
- [How `.env` files are parsed](docs/parsing.md)
- [Settings](docs/settings.md)
- [Development](docs/development.md)

## License

No license has been chosen yet.
