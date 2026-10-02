---
title: Settings
---

# Settings

[Home](index.md) · [Usage](usage.md) · [Parsing](parsing.md) · [Development](development.md)

Click **Settings** at the top right of the left panel to open the settings
panel. Changes apply immediately and are saved automatically.

## Dark mode

Tick **Dark mode** to switch between the light and dark themes.

## Font family

The **Font family** dropdown lists the fonts installed on your machine. Type in
its search box to filter the list, then pick a font. It is applied to the whole
window: table, name fields and text areas.

Use the clear (×) button to go back to the default monospace font.

The list contains every installed font, not only monospace ones, because the app
cannot tell which are monospace. A proportional font works but makes character
differences harder to see.

## Where settings are stored

Settings are a small JSON file in the platform configuration directory:

| Platform | Path                                                   |
| -------- | ------------------------------------------------------ |
| macOS    | `~/Library/Application Support/env-compare/settings.json` |
| Linux    | `~/.config/env-compare/settings.json`                  |
| Windows  | `%APPDATA%\env-compare\settings.json`                  |

```json
{
  "dark_mode": true,
  "font_family": "JetBrains Mono"
}
```

An empty `font_family` means the default monospace font. If the file is missing
or unreadable, defaults are used. Delete it to reset everything.

Pasted text and loaded files are never saved; only these two preferences are.
