# Module Dependency Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expose parsed `include` declarations as module dependencies in MIR.

**Architecture:** Keep the existing `include "./path.fkl"` grammar and lower include paths into `ContextMap.module_dependencies`.

**Tech Stack:** Rust workspace crates `fkl_parser`, `fkl_mir`, `cargo test`.

---

### Task 1: Include Dependencies In MIR

**Files:**
- Modify: `fkl_mir/src/strategy/context_map.rs`
- Modify: `fkl_parser/src/parser/parser.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing test**

Add a transform test showing two include declarations become `module_dependencies`.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_parser module_dependencies_from_includes`

Expected: FAIL because MIR does not expose module dependencies yet.

- [x] **Step 3: Implement MIR field and transform lowering**

Add `module_dependencies: Vec<String>` to `ContextMap` and push include paths during transform.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_parser module_dependencies_from_includes
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-module-dependency.md fkl_mir/src/strategy/context_map.rs fkl_parser/src/parser/parser.rs fkl_parser/src/tests.rs fkl_parser/src/transform.rs README.md
git commit -m "feat(module): expose include dependencies"
git push origin master
```
