# Open GitHub Issues Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the currently open issues from `https://github.com/feakin/fklang/issues` in small, tested, pushed slices.

**Architecture:** Keep each issue as a separate vertical slice with its own tests and commit. Prefer existing crate boundaries: CLI features in `fkl_cli`, syntax and MIR changes in `fkl_parser` and `fkl_mir`, generation in `fkl_codegen_*`, extension loading in `fkl_ext_*` and `extensions/*`.

**Tech Stack:** Rust 2021, Cargo workspace, Clap CLI, Pest parser, existing MIR crates, GitHub issue list from `gh issue list --repo feakin/fklang --state open`.

---

## Current Open Issues

- #7 `Init by template`: add a CLI template initializer.
- #1 `Type system for FKL`: add explicit type definition support, starting with constrained aliases.
- #4 `Feakin Code Generator`: extend code generation beyond REST API, starting with SQL schema generation.
- #10 `Plugin system`: make plugin registry/discovery usable for small extension packages.
- #12 `Scene: Environment checking DSL and MVP code`: add an MVP environment checking scene around existing `env` parsing and connection runners.
- #13 `Computation ext`: make the computation extension build and support simple expression/math evaluation.

## Execution Order

### Task 1: Issue #7 Init By Template

**Files:**
- Modify: `fkl_cli/src/main.rs`
- Create: `fkl_cli/src/init.rs`
- Test: `fkl_cli/src/init.rs`
- Docs: `fkl_cli/README.md`

- [x] **Step 1: Write failing tests**

Add tests proving:
- `init_project` creates `main.fkl` in an empty directory.
- `init_project` returns an already-exists error when `main.fkl` exists and overwrite is false.

- [x] **Step 2: Verify red**

Run: `cargo test -p fkl_cli init::tests`

Expected: compile failure or test failure because `init_project` does not exist.

- [x] **Step 3: Implement minimal CLI init**

Add `fkl init --path <dir> --name <Name> [--force]` that writes a parseable starter `main.fkl`.

- [x] **Step 4: Verify green**

Run: `cargo test -p fkl_cli init::tests`

Expected: tests pass.

- [ ] **Step 5: Commit and push**

Commit: `feat(cli): add init template command`

### Task 2: Issue #1 Type System MVP

**Files:**
- Modify: `fkl_parser/src/parser/fkl.pest`
- Modify: `fkl_parser/src/parser/ast.rs`
- Modify: `fkl_parser/src/parser/parser.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `fkl_mir/src/lib.rs`
- Test: parser and MIR transform tests near existing parser tests.

- [ ] Add failing parser tests for `type Percent = Int range 0..100;`.
- [ ] Add AST and MIR structures for named type aliases with optional numeric range constraints.
- [ ] Transform parsed type declarations into `ContextMap.types`.
- [ ] Verify with `cargo test -p fkl_parser type`.
- [ ] Commit and push as `feat(parser): add type alias declarations`.

### Task 3: Issue #4 SQL Generator MVP

**Files:**
- Create: `fkl_codegen_sql/src/lib.rs`
- Create: `fkl_codegen_sql/Cargo.toml`
- Modify: root `Cargo.toml`
- Modify: `fkl_cli/src/main.rs`
- Test: SQL generator unit tests and a CLI-level smoke test.

- [ ] Add failing tests for mapping an entity with `String`, `Int`, and `UUID` fields to `CREATE TABLE`.
- [ ] Implement a small SQL generator crate from existing MIR entities.
- [ ] Add `fkl gen --framework sql`.
- [ ] Verify with `cargo test -p fkl_codegen_sql` and focused CLI tests.
- [ ] Commit and push as `feat(codegen): add sql schema generator`.

### Task 4: Issue #10 Plugin Registry MVP

**Files:**
- Modify: `fkl_ext_loader/src/lib.rs`
- Modify: `fkl_cli/src/main.rs`
- Test: `fkl_ext_loader` unit tests.

- [ ] Add failing tests for loading plugin manifests from a registry directory.
- [ ] Implement a manifest type with name, path, and extension kind.
- [ ] Add CLI listing for available plugins.
- [ ] Verify with `cargo test -p fkl_ext_loader`.
- [ ] Commit and push as `feat(plugin): add local plugin registry`.

### Task 5: Issue #12 Environment Checking DSL MVP

**Files:**
- Modify: `fkl_parser/src/parser/fkl.pest`
- Modify: `fkl_parser/src/parser/ast.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `fkl_cli/src/builtin/funcs`
- Test: parser and CLI built-in unit tests.

- [ ] Add failing tests for a `check` block under `env`.
- [ ] Parse and transform checks into MIR.
- [ ] Reuse existing datasource/server/custom environment runners to execute checks.
- [ ] Verify with focused parser and CLI tests.
- [ ] Commit and push as `feat(env): add environment check scene`.

### Task 6: Issue #13 Computation Extension MVP

**Files:**
- Modify: `extensions/ext_computing/Cargo.toml`
- Modify: `extensions/ext_computing/src/lib.rs`
- Modify: `extensions/ext_computing/src/function_type.rs`
- Test: `extensions/ext_computing` unit tests.

- [ ] Replace the unresolved `salsa-2022` git dependency with a buildable expression evaluator path.
- [ ] Add failing tests for integer addition, division precedence, and `sum` over numeric values.
- [ ] Implement expression evaluation behind the extension API.
- [ ] Re-include `extensions/ext_computing` in the workspace once it builds.
- [ ] Verify with `cargo test -p fkl_ext_computing` and `cargo test -p fkl_lsp`.
- [ ] Commit and push as `feat(computing): add simple math evaluator`.

## Verification Policy

Every issue slice must:
- add or update tests before implementation;
- run a focused `cargo test -p ...` command;
- commit only the files for that issue;
- push immediately after the commit.
