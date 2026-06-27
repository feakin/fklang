# Better Code Generation Implementation Plan

**Goal:** Improve generated Spring entity code so generated domain entities are directly usable by Java frameworks and application code.

**Architecture:** Keep generation in `fkl_codegen_java::spring_gen::entity_gen`; extend the existing field-only entity output with a public no-args constructor and getter/setter methods for each field.

**Tech Stack:** Rust workspace crates `fkl_codegen_java`, `fkl_cli`, `cargo test`.

---

### Task 1: Spring Entity Accessors

**Files:**
- Modify: `fkl_codegen_java/src/spring_gen/entity_gen.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing test**

Add a focused test asserting generated Spring entity code includes a no-args constructor plus getter and setter methods for fields.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_codegen_java generates_accessors_for_spring_jpa_entity`

Expected: FAIL because entity generation currently emits only fields.

- [x] **Step 3: Implement accessors**

Generate no-args constructors and field accessors while preserving existing imports, `@Entity`, `@Id`, and type mapping behavior.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_codegen_java generates_accessors_for_spring_jpa_entity
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-better-code-generation.md fkl_codegen_java/src/spring_gen/entity_gen.rs README.md
git commit -m "feat(codegen): add spring entity accessors"
git push origin master
```
