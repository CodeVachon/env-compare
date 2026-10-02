---
title: ENV Compare
---

# ENV Compare

A small desktop app for comparing `.env` files side by side. Paste the contents
of two or more env files, or open the files themselves, and get one table with
a row per key and a column per environment.

|           | Env1                   | Env2                   |
| --------- | ---------------------- | ---------------------- |
| `API_KEY` | `djfjfkdkd`            | `sldkfjslkd`           |
| `SECRET`  | `dfgdfgdf`             | `dfgfgdf`              |
| `URL`     | `https://api.env1.com` | `https://api.env2.com` |

## Why

Config drift between environments is easy to miss by eye, especially in long
API keys where one character is wrong. ENV Compare lines the files up, can hide
everything that already matches, and can mark the exact characters that differ.

## Features

- Paste, open, or drag in any number of `.env` files
- Show only differences
- Character-level difference highlighting
- Dark and light mode
- Custom font, chosen from your installed fonts
- Preferences are remembered between launches

## Get started

```sh
git clone https://github.com/CodeVachon/env-compare.git
cd env-compare
cargo run --release
```

Requires a recent stable [Rust toolchain](https://rustup.rs/). Built and run on
macOS; Linux and Windows are untested.

## Read more

- [Usage](usage.md)
- [How `.env` files are parsed](parsing.md)
- [Settings](settings.md)
- [Development](development.md)
