# FKL LSP

FKL language server implemented with `tower-lsp`.

## Run

```bash
cargo run -p fkl_lsp
```

The server speaks the Language Server Protocol over stdio.

## Capabilities

- full text document sync
- syntax diagnostics powered by `fkl_parser`
- keyword and snippet completions
- keyword hover documentation

Monaco sample reference: https://github.com/silvanshade/tower-lsp-web-demo
