# AWS Codegen Plugin MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn `ext_codegen_aws` from a placeholder crate into a loadable codegen plugin that can generate an AWS Lambda Java handler skeleton.

**Architecture:** Keep the MVP self-contained in `extensions/ext_codegen_aws`. Implement `CustomRunner` with a pure handler generator so behavior is unit-testable without AWS SDK dependencies or network access.

**Tech Stack:** Rust workspace crate `fkl_ext_codegen_aws`, `fkl_ext_api::CustomRunner`, `fkl_mir`, `async-trait`, `cargo test`.

---

### Task 1: Lambda Handler Command

**Files:**
- Modify: `extensions/ext_codegen_aws/Cargo.toml`
- Modify: `extensions/ext_codegen_aws/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write the failing tests**

Add tests that assert the runner name is `aws-codegen`, lists `lambda-handler`, returns a Java Lambda handler with package and handler arguments, and ignores unknown commands.

- [x] **Step 2: Run tests to verify they fail**

Run: `cargo test -p fkl_ext_codegen_aws`

Expected: FAIL because the crate currently only exposes placeholder `add()`.

- [x] **Step 3: Write minimal implementation**

Implement `AwsCodegenRunner`, `_fkl_create_runner`, `generate_lambda_handler`, and the `lambda-handler` command.

- [x] **Step 4: Run focused tests**

Run: `cargo test -p fkl_ext_codegen_aws`

Expected: PASS.

- [x] **Step 5: Run workspace tests**

Run: `cargo test --all`

Expected: PASS.

- [ ] **Step 6: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-aws-codegen-plugin-mvp.md extensions/ext_codegen_aws/Cargo.toml extensions/ext_codegen_aws/src/lib.rs README.md
git commit -m "feat(aws): add lambda codegen plugin"
git push origin master
```
