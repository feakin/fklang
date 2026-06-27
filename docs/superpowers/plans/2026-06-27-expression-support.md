# Expression Support Implementation Plan

**Goal:** Add minimal expression statements to function bodies.

**Architecture:** Parse function body statements as raw expression source and lower them into MIR. Start with `return <expr>;` and plain `<expr>;` statements without evaluation or type checking.

**Tech Stack:** Rust workspace crates `fkl_parser`, `fkl_mir`, `cargo test`.

---

### Task 1: Function Body Expressions

**Files:**
- Modify: `fkl_parser/src/parser/fkl.pest`
- Modify: `fkl_parser/src/parser/ast.rs`
- Modify: `fkl_parser/src/parser/parser.rs`
- Modify: `fkl_mir/src/strategy/context_map.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing test**

Add a transform test showing a function return expression is available in MIR.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_parser function_return_expressions_lower_to_mir`

Expected: FAIL because function bodies currently accept no expression statements.

- [x] **Step 3: Implement expression parsing and MIR lowering**

Add return/plain expression statement grammar, AST nodes, MIR nodes, and transform lowering.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_parser function_return_expressions_lower_to_mir
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-expression-support.md fkl_parser/src/parser/fkl.pest fkl_parser/src/parser/ast.rs fkl_parser/src/parser/parser.rs fkl_mir/src/strategy/context_map.rs fkl_parser/src/transform.rs README.md
git commit -m "feat(language): add function expressions"
git push origin master
```
