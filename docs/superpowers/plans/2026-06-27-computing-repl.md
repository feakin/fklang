# Computing REPL Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a minimal REPL session core to the computing extension.

**Architecture:** Keep the interactive shell boundary small by adding a reusable line-oriented REPL evaluator in `fkl_ext_computing`. Each non-empty line is evaluated with the existing expression evaluator until `exit` or `quit`.

**Tech Stack:** Rust workspace crate `fkl_ext_computing`, `cargo test`.

---

### Task 1: Line-Oriented REPL

**Files:**
- Modify: `extensions/ext_computing/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing tests**

Add tests for line-oriented REPL evaluation and for listing the `repl` command.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_ext_computing repl`

Expected: FAIL because REPL support does not exist yet.

- [x] **Step 3: Implement minimal REPL support**

Add `run_repl_lines` and expose `repl` in the computing runner command list. The `repl` command accepts one multiline argument and returns newline-separated output.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_ext_computing repl
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-computing-repl.md extensions/ext_computing/src/lib.rs README.md
git commit -m "feat(computing): add repl session core"
git push origin master
```
