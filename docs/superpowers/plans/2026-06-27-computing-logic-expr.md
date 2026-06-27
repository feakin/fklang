# Computing Logic Expression Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add basic logic expression support to the computing extension.

**Architecture:** Extend the existing in-crate recursive descent evaluator so logical expressions keep the current `f64` public API and return `1.0` for true and `0.0` for false.

**Tech Stack:** Rust workspace crate `fkl_ext_computing`, `cargo test`.

---

### Task 1: Logic Expressions

**Files:**
- Modify: `extensions/ext_computing/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing tests**

Add tests for boolean literals, comparison operators, `&&`, `||`, and unary `!`.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_ext_computing logic`

Expected: FAIL because logic tokens and parser precedence do not exist yet.

- [x] **Step 3: Implement minimal parser support**

Add tokens and parser precedence layers for logical-or, logical-and, equality, comparison, additive, multiplicative, and unary expressions.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_ext_computing logic
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-computing-logic-expr.md extensions/ext_computing/src/lib.rs README.md
git commit -m "feat(computing): evaluate logic expressions"
git push origin master
```
