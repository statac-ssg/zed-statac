# Statac for Zed

[Statac](https://statac.dev)'s language server in Zed. In a Statac site,
Zed shows each problem `statac check` finds as you type, and offers
completion, hover, go to definition, find references and fixes. When you
rename a page's file in the project panel, the links to it are updated.

It needs Statac 0.7.0 or later. With an earlier Statac, the language
server doesn't start.

## Installing

1. Install [Statac](https://statac.dev/install/).
2. In Zed, run zed: extensions, search for Statac, and install it.
3. Open a folder that holds a Statac site: a `statac.yaml` file, or a
   `content` folder.

## Settings

Zed looks for `statac` on your shell's path. To use another, give its
whole path in Zed's settings:

```json
{
  "lsp": {
    "statac": {
      "binary": { "path": "/path/to/statac" }
    }
  }
}
```

## More

- [Editors](https://statac.dev/editors/), in Statac's documentation.
- [Issues](https://git.statac.dev/statac/zed-statac/issues).

## Licence

MIT or Apache-2.0, at your choice: [LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE).
