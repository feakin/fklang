# Swagger SourceSet Plugin MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn `ext_sourceset_swagger` into a loadable plugin that generates a valid FKL `SourceSet` block for Swagger/OpenAPI inputs.

**Architecture:** Add a library target alongside the existing binary. The plugin exposes a pure `source-set` command, and tests verify both the command surface and that generated FKL parses with the existing parser.

**Tech Stack:** Rust workspace crate `fkl_ext_sourceset_swagger`, `fkl_ext_api::CustomRunner`, `fkl_parser`, `async-trait`, `cargo test`.

---

### Task 1: Swagger SourceSet Command

**Files:**
- Modify: `extensions/ext_sourceset_swagger/Cargo.toml`
- Create: `extensions/ext_sourceset_swagger/src/lib.rs`
- Modify: `README.md`

- [x] **Step 1: Write the failing tests**

Add tests that assert the runner name is `sourceset-swagger`, lists `source-set`, returns a Swagger `SourceSet` FKL snippet from arguments, and that the snippet parses.

- [x] **Step 2: Run tests to verify they fail**

Run: `cargo test -p fkl_ext_sourceset_swagger`

Expected: FAIL because the crate currently has no library runner.

- [x] **Step 3: Write minimal implementation**

Implement `SwaggerSourceSetRunner`, `_fkl_create_runner`, `generate_source_set`, and the `source-set` command.

- [x] **Step 4: Run focused tests**

Run: `cargo test -p fkl_ext_sourceset_swagger`

Expected: PASS.

- [x] **Step 5: Run workspace tests**

Run: `cargo test --all`

Expected: PASS.

- [ ] **Step 6: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-swagger-sourceset-plugin-mvp.md extensions/ext_sourceset_swagger/Cargo.toml extensions/ext_sourceset_swagger/src/lib.rs README.md
git commit -m "feat(swagger): add sourceset plugin"
git push origin master
```
