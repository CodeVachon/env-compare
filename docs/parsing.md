---
title: Parsing
---

# How `.env` files are parsed

[Home](index.md) · [Usage](usage.md) · [Settings](settings.md) · [Development](development.md)

The parser is deliberately small and forgiving. It lives in `src/parser.rs` and
has no dependency on the UI.

| Input                       | Result                                          |
| --------------------------- | ----------------------------------------------- |
| `KEY=value`                 | `KEY` → `value`                                 |
| `export KEY=value`          | `export ` prefix is ignored                     |
| `KEY = value`               | whitespace around the key and value is trimmed  |
| `KEY="hello world"`         | double quotes removed: `hello world`            |
| `KEY='x=y'`                 | single quotes removed: `x=y`                    |
| `KEY=plain # note`          | trailing ` # comment` removed: `plain`          |
| `KEY=a#b`                   | kept as is (no space before `#`): `a#b`         |
| `URL=https://x.com/?a=1`    | only the first `=` splits: `https://x.com/?a=1` |
| `# comment`                 | skipped                                         |
| blank line                  | skipped                                         |
| `not a pair`                | skipped (no `=`)                                |
| `=value`                    | skipped (empty key)                             |

If a key appears more than once in a file, the last value wins.

## Limitations

- No variable expansion (`${OTHER}` is kept literally).
- No multi-line values. Each line is parsed on its own, so a quoted value that
  continues on the next line has no closing quote on its first line and is kept
  as written, opening quote included. The continuation lines are skipped.
- Escape sequences inside quotes (such as `\n`) are not interpreted.

## Comparing

After parsing, the environments are merged into one row per key, using the union
of all keys. A row "differs" when the environments do not all hold the same
value, including when a key is missing from one of them.
