# SQL Migration Codegen Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a minimal database migration output to SQL code generation.

**Architecture:** Reuse the existing SQL schema generator output and have the CLI write it both as `schema.sql` and as the initial versioned migration file under `migrations/V1__init.sql`.

**Tech Stack:** Rust workspace crates `fkl_cli`, `fkl_codegen_sql`, `cargo test`.

---

### Task 1: Initial SQL Migration File

**Files:**
- Modify: `fkl_cli/src/builtin/funcs/code_gen/mod.rs`
- Modify: `README.md`

- [x] **Step 1: Write the failing test**

Extend the existing SQL framework test to assert `migrations/V1__init.sql` is written with the generated schema content.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_cli sql_framework_writes_schema_file`

Expected: FAIL because the migration file is not written yet.

- [x] **Step 3: Write minimal implementation**

In the `SupportedFramework::Sql` branch, create the `migrations` directory and write `V1__init.sql` with the same generated schema.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_cli sql_framework_writes_schema_file
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-sql-migration-codegen.md fkl_cli/src/builtin/funcs/code_gen/mod.rs README.md
git commit -m "feat(sql): write initial migration file"
git push origin master
```
