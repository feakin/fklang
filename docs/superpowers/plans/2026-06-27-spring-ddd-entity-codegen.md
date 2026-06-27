# Spring DDD Entity Codegen Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the Spring code generator produce DDD entity classes from MIR entities.

**Architecture:** Add a pure Java entity generator in `fkl_codegen_java`, then call it from the existing CLI `gen --framework spring` path. Entity files are written to the configured domain layer package when `LayeredArchitecture` is present, or to `src/main/java/domain` as a fallback.

**Tech Stack:** Rust workspace crates `fkl_codegen_java`, `fkl_cli`, `fkl_mir`, `cargo test`.

---

### Task 1: Spring JPA Entity Generator

**Files:**
- Modify: `fkl_codegen_java/src/spring_gen/entity_gen.rs`
- Modify: `fkl_codegen_java/src/lib.rs`

- [x] **Step 1: Write the failing tests**

Add tests that assert a MIR `Entity` generates a Java class with `package`, `@Entity`, `@Id`, and typed fields.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_codegen_java spring_gen::entity_gen`

Expected: FAIL because `gen_spring_entity` is not implemented.

- [x] **Step 3: Write minimal implementation**

Implement `gen_spring_entity(entity, package)` and type mapping for `UUID`, `String`, `Int`, `Long`, `Float`, `Double`, `Boolean`, and default custom types.

- [x] **Step 4: Run focused test**

Run: `cargo test -p fkl_codegen_java spring_gen::entity_gen`

Expected: PASS.

### Task 2: CLI Writes Spring Domain Entities

**Files:**
- Modify: `fkl_cli/src/builtin/funcs/code_gen/layer_path_builder.rs`
- Modify: `fkl_cli/src/builtin/funcs/code_gen/mod.rs`
- Modify: `README.md`

- [x] **Step 1: Write the failing CLI test**

Add a test that calls `code_gen_by_mir` with `SupportedFramework::Spring` and asserts `src/main/java/domain/Ticket.java` is written.

- [x] **Step 2: Run focused CLI test to verify it fails**

Run: `cargo test -p fkl_cli spring_framework_writes_domain_entity_files`

Expected: FAIL because Spring codegen currently only handles HTTP controller snippets.

- [x] **Step 3: Write minimal implementation**

Write all entities under the domain package before existing controller insertion logic.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_codegen_java spring_gen::entity_gen
cargo test -p fkl_cli spring_framework_writes_domain_entity_files
cargo test --all
```

Expected: PASS.

- [ ] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-spring-ddd-entity-codegen.md fkl_codegen_java/src/spring_gen/entity_gen.rs fkl_codegen_java/src/lib.rs fkl_cli/src/builtin/funcs/code_gen/layer_path_builder.rs fkl_cli/src/builtin/funcs/code_gen/mod.rs README.md
git commit -m "feat(spring): generate ddd entity classes"
git push origin master
```
