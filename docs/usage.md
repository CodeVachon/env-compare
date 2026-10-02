---
title: Usage
---

# Usage

[Home](index.md) · [Parsing](parsing.md) · [Settings](settings.md) · [Development](development.md)

## Layout

The window has two parts:

- **Left panel:** the controls and one card per environment. Each card has a
  name field and a text area for the file contents.
- **Right side:** the comparison table. The first column is the key; every
  other column is one environment, headed by its name.

The table updates as you type or paste.

## Adding environments

The app starts with two empty environments. You can:

- **Paste** the contents of a `.env` file into a card's text area.
- Click **Add environment** for another card.
- Click **Open files…** and select one or more files.
- **Drag files** from Finder onto the window.

Loaded files are named after the file, for example `.env.production`. If two
files in the same batch share a name, the parent folder is added
(`staging/.env`). Rename any environment by editing its name field.

Loading files replaces any environments that are still empty, so you don't end
up with blank columns beside your files. Environments you've already filled in
are kept.

> **macOS tip:** the file picker hides dotfiles such as `.env` by default. Press
> **Cmd+Shift+.** in the picker to show them, or drag the files in from Finder.

Files that can't be read as text are skipped without an error message.

## Removing and clearing

- **Clear** empties an environment's contents but keeps its card, name and
  column, ready for a new paste.
- **Remove** deletes the environment and its column. It only appears when more
  than two environments exist.

## Reading the table

- A key that an environment doesn't define shows `—` in that cell.
- Keys are sorted alphabetically.
- Values in rows where the environments disagree are drawn in the theme's
  warning colour.

## Show only differences

Tick **Show only differences** to hide every row where all environments have the
same value. A key missing from one environment counts as a difference, so it
stays visible.

## Highlight differences

Tick **Highlight differences** to mark exactly which characters differ. Each
character in a differing row is compared with the character at the same
position in the other environments; mismatches get a tinted background and bold
text. This makes look-alike characters such as `0` and `O` easy to spot.

Comparison is by position. If one value has an extra character near the start,
everything after it is marked as different even if the rest matches. For API
keys and tokens that is usually what you want.
