# RouterOS Script for Zed

MikroTik **RouterOS** `.rsc` support for [Zed](https://zed.dev) — configuration
exports and scripts alike.

Everything works **offline**. No router, no credentials, no network.

## Features

**Syntax highlighting** via [tree-sitter-routeros](https://github.com/DamirManyapov/tree-sitter-routeros):
paths, commands, properties, strings, variables and comments, plus outline
navigation by config section (`cmd-shift-o`), folding, bracket matching and
auto-indent.

**Completion and diagnostics** via [routeros-lsp](https://github.com/DamirManyapov/routeros-lsp):

- `/interface/wire` → `wireguard`, `wireless`
- `/interface/wireguard/add ` → `mtu=`, `private-key=`, `vrf=`
- `/interface/wi` is flagged as an unknown path segment
- completion notes when a parameter is newer than most releases (`vrf` · `7.21+`)

The command tree is bundled with the language server, merged from 60 RouterOS
releases (7.9 through 7.24), so completion is not tied to any one version.

## Handled quirks

RouterOS exports break most generic highlighters. This one handles:

- backslash line continuations, including ones that split a quoted string
- CRLF line endings, which every export uses
- nested properties: `channel.frequency=5180`, and a leading dot (`.mode=ap`)
  continuing the previous group
- both path spellings — `/ip firewall filter` and `/ip/firewall/filter` — with
  an optional inline command
- slashes inside values, so `address=0.0.0.0/8` stays a value

## Not included

Value validation. The upstream schema carries parameter names but no types or
ranges, so `mtu=78000` is not flagged.

## Turning the language server off

Highlighting is pure tree-sitter and never depends on the language server. To
run without completion:

```json
{
  "lsp": {
    "routeros-lsp": { "enabled": false }
  }
}
```

## Installation

Zed → Extensions → search for **RouterOS**.

## Development

```bash
git clone https://github.com/DamirManyapov/zed-routeros
cd zed-routeros
cargo build --release --target wasm32-wasip1
```

Then in Zed: `cmd-shift-p` → `zed: install dev extension` → pick the folder.

## License

MIT
