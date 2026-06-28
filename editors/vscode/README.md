# FKL VSCode Extension

VSCode language support for `.fkl` files backed by the Rust `fkl_lsp` server.

## Development

```bash
cargo build -p fkl_lsp
cd editors/vscode
npm install
npm test
```

By default the extension starts `../../target/debug/fkl_lsp` relative to `editors/vscode`. Set `fkl.lsp.serverPath` to use another binary.
