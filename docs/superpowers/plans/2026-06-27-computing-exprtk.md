# Computing ExprTk Integration Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a minimal Rust ExprTk-compatible expression path to the computing extension.

**Architecture:** Keep CI portable by using the in-repo evaluator for a small ExprTk-compatible subset exposed through a wrapper function and `exprtk` custom command. The native `exprtk_rs` crate was checked first, but its bundled C++ binding fails on the current macOS toolchain.

**Tech Stack:** Rust workspace crate `fkl_ext_computing`, `cargo test`.

---

### Task 1: ExprTk Backend

**Files:**
- Modify: `extensions/ext_computing/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing tests**

Add tests for evaluating an ExprTk-compatible expression with variables and for listing the `exprtk` command.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_ext_computing exprtk`

Expected: FAIL because ExprTk-compatible wrapper support does not exist yet.

- [x] **Step 3: Implement minimal ExprTk-compatible wrapper**

Bind numeric variables into the existing evaluator, add `^` power support, and expose an `exprtk` command.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_ext_computing exprtk
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-computing-exprtk.md extensions/ext_computing/src/lib.rs README.md
git commit -m "feat(computing): add exprtk backend"
git push origin master
```
