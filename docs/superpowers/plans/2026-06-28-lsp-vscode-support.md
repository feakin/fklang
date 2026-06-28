# LSP VSCode Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve the existing `fkl_lsp` editor surface and add a VSCode extension that launches the Rust LSP over stdio.

**Architecture:** Keep the language intelligence in the existing Rust `fkl_lsp` crate. Add document symbols and updated keyword/snippet support there, then add a small VSCode extension under `editors/vscode` that depends on `vscode-languageclient` and resolves either a configured server path or the workspace-built `fkl_lsp` binary.

**Tech Stack:** Rust `tower-lsp`, `fkl_parser`, VSCode extension API, TypeScript, npm.

---

### Task 1: Improve FKL LSP Editor Capabilities

**Files:**
- Modify: `fkl_lsp/src/lib.rs`
- Modify: `fkl_lsp/README.md`

- [x] **Step 1: Write failing tests**

Add tests proving:
- `server_capabilities()` advertises a document symbol provider.
- `completion_items()` includes newer language snippets such as `include`, `function`, `return`, and `type`.
- `document_symbols_for_text()` returns outline entries for top-level declarations.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_lsp document_symbols`

Expected: FAIL because document symbols are not implemented.

- [x] **Step 3: Implement LSP support**

Add `textDocument/documentSymbol` handling, a public `document_symbols_for_text()` helper, updated completion entries, and README capability notes.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_lsp
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-28-lsp-vscode-support.md fkl_lsp/src/lib.rs fkl_lsp/README.md
git commit -m "feat(lsp): add document symbols"
git push origin master
```

### Task 2: Add VSCode LSP Extension

**Files:**
- Create: `editors/vscode/package.json`
- Create: `editors/vscode/tsconfig.json`
- Create: `editors/vscode/src/extension.ts`
- Create: `editors/vscode/test/extension.test.ts`
- Create: `editors/vscode/README.md`
- Modify: `.gitignore`
- Modify: `README.md`
- Modify: `.github/workflows/build.yml`

- [x] **Step 1: Write failing extension tests**

Add tests for resolving a configured server path and fallback workspace binary path.

- [x] **Step 2: Run focused test to verify it fails**

Run:
```bash
cd editors/vscode
npm install
npm test
```

Expected: FAIL until extension resolver code exists.

- [x] **Step 3: Implement extension**

Add activation for `fkl` files, a `vscode-languageclient` stdio client, server path resolution, and a configuration key `fkl.lsp.serverPath`.

- [x] **Step 4: Run extension and workspace verification**

Run:
```bash
cd editors/vscode
npm test
npm run compile
cargo test -p fkl_lsp
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add editors/vscode .gitignore README.md docs/superpowers/plans/2026-06-28-lsp-vscode-support.md
git commit -m "feat(vscode): add fkl lsp extension"
git push origin master
```
