# Nim

Nim language support in Zed is provided by the community-maintained [Nim extension](https://tvv.tw/https://github.com/foxoman/zed-nim).
Report issues to: [https://tvv.tw/https://github.com/foxoman/zed-nim/issues](https://tvv.tw/https://github.com/foxoman/zed-nim/issues)

- Tree-sitter: [alaviss/tree-sitter-nim](https://tvv.tw/https://github.com/alaviss/tree-sitter-nim)
- Language Server: [nim-lang/langserver](https://tvv.tw/https://github.com/nim-lang/langserver)

## Formatting

To use [arnetheduck/nph](https://tvv.tw/https://github.com/arnetheduck/nph) as a formatter, follow the [nph installation instructions](https://tvv.tw/https://github.com/arnetheduck/nph?tab=readme-ov-file#installation) and add this to your Zed `settings.json`:

```json
  "languages": {
    "Nim": {
      "formatter": {
        "external": {
          "command": "nph",
          "arguments": ["-"]
        }
      }
    }
  }
```
