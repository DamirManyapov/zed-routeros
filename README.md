# RouterOS Script for Zed

Syntax highlighting for MikroTik **RouterOS** `.rsc` files — configuration
exports and scripts alike.

Built on [tree-sitter-routeros](https://github.com/DamirManyapov/tree-sitter-routeros).

## Features

- Syntax highlighting for paths, commands, properties, strings and variables
- Outline (`cmd-shift-o`) that lists config sections such as `/ip firewall filter`
- Code folding for `do={ ... }` blocks and `[ ... ]` substitutions
- Bracket matching and auto-indent
- Hyphenated property names (`allowed-address`, `in-interface-list`) treated as
  single words for selection and navigation

Handles the parts of the format that usually break highlighters: backslash line
continuations — including ones that split a quoted string in half — CRLF
endings, nested properties (`channel.frequency=`, `.mode=`), both `/ip firewall
filter` and `/ip/firewall/filter` spellings, and slashes inside values such as
`address=0.0.0.0/8`.

## Installation

Zed → Extensions → search for **RouterOS**.

## Development

```bash
git clone https://github.com/DamirManyapov/zed-routeros
```

Then in Zed: `cmd-shift-p` → `zed: install dev extension` → pick the folder.

## License

MIT
