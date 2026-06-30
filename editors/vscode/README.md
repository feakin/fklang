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

Run the VSCode Extension Host integration check with:

```bash
npm run test:integration
```

The integration check opens a real VSCode development host, launches the `FKL Time Travel Debug`
configuration against `docs/samples/impl.fkl`, and verifies the debug session trace, stack frame,
and seek request. Set `FKL_VSCODE_EXECUTABLE` when the VSCode executable is not in the default
macOS location.

## Time Travel Debug

```bash
cargo build -p fkl_cli
```

Use the `FKL Time Travel Debug` launch configuration for `.fkl` files. By default the extension starts
`../../target/debug/fkl dap --main ${file}` relative to `editors/vscode`. Set `fkl.debug.adapterPath`
to use another `fkl` CLI binary.
