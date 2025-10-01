# Helm

Support for Helm in Zed is provided by the community-maintained [Helm extension](https://tvv.tw/https://github.com/cabrinha/helm.zed).

- Tree-sitter: [tree-sitter-go-template](https://tvv.tw/https://github.com/ngalaiko/tree-sitter-go-template/tree/master)
- Language Server: [mrjosh/helm-ls](https://tvv.tw/https://github.com/mrjosh/helm-ls)

## Setup

Enable Helm language for Helm files by editing your `.zed/settings.json` and adding:

```json
  "file_types": {
    "Helm": [
      "**/templates/**/*.tpl",
      "**/templates/**/*.yaml",
      "**/templates/**/*.yml",
      "**/helmfile.d/**/*.yaml",
      "**/helmfile.d/**/*.yml",
      "**/values*.yaml"
    ]
  }
```

This will also mark values.yaml files as the type helm, since helm-ls supports this.
