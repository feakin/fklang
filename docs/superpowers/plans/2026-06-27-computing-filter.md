# Computing Filter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a minimal filter capability to the computing extension.

**Architecture:** Reuse the existing expression evaluator as a numeric predicate evaluator. The `filter` command accepts a comma-separated numeric list and a predicate expression where `x` is the current item.

**Tech Stack:** Rust workspace crate `fkl_ext_computing`, `cargo test`.

---

### Task 1: Numeric Filter

**Files:**
- Modify: `extensions/ext_computing/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing tests**

Add tests for filtering comma-separated numeric values with an `x` predicate and for listing the `filter` command.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_ext_computing filter`

Expected: FAIL because filter support does not exist yet.

- [x] **Step 3: Implement minimal filter support**

Add variable-aware evaluation for `x`, parse numeric CSV input, and expose `filter` in the computing runner command list.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_ext_computing filter
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-computing-filter.md extensions/ext_computing/src/lib.rs README.md
git commit -m "feat(computing): filter numeric values"
git push origin master
```
