# Module Versioning Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add optional version metadata for module include dependencies.

**Architecture:** Extend existing `include "./path.fkl"` syntax with optional `version "x.y.z"` and lower versions into `ContextMap.module_versions`.

**Tech Stack:** Rust workspace crates `fkl_parser`, `fkl_mir`, `cargo test`.

---

### Task 1: Include Dependency Versions

**Files:**
- Modify: `fkl_parser/src/parser/fkl.pest`
- Modify: `fkl_parser/src/parser/ast.rs`
- Modify: `fkl_parser/src/parser/parser.rs`
- Modify: `fkl_mir/src/strategy/context_map.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing test**

Add a transform test showing an include declaration with `version` becomes a module version entry.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_parser module_versions_from_includes`

Expected: FAIL because include version syntax and MIR lowering do not exist yet.

- [x] **Step 3: Implement parser and MIR lowering**

Add optional include version parsing, `module_versions` to MIR, and transform lowering.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_parser module_versions_from_includes
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-module-versioning.md fkl_parser/src/parser/fkl.pest fkl_parser/src/parser/ast.rs fkl_parser/src/parser/parser.rs fkl_mir/src/strategy/context_map.rs fkl_parser/src/tests.rs fkl_parser/src/transform.rs README.md
git commit -m "feat(module): parse include versions"
git push origin master
```
