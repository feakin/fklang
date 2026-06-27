# Function Support Implementation Plan

**Goal:** Add first-class top-level function declarations to the parser and MIR.

**Architecture:** Parse `function name(args) -> Return {}` / `fn name(args) -> Return {}` as declarations and lower signatures into `ContextMap.functions`.

**Tech Stack:** Rust workspace crates `fkl_parser`, `fkl_mir`, `cargo test`.

---

### Task 1: Function Signatures

**Files:**
- Modify: `fkl_parser/src/parser/fkl.pest`
- Modify: `fkl_parser/src/parser/ast.rs`
- Modify: `fkl_parser/src/parser/parser.rs`
- Modify: `fkl_mir/src/strategy/context_map.rs`
- Modify: `fkl_parser/src/transform.rs`
- Modify: `fkl_parser/src/tests.rs`
- Modify: `README.md`

- [x] **Step 1: Write failing test**

Add a transform test showing a top-level function declaration is available in MIR.

- [x] **Step 2: Run focused test to verify it fails**

Run: `cargo test -p fkl_parser function_signatures_lower_to_mir`

Expected: FAIL because parser/MIR function support does not exist yet.

- [x] **Step 3: Implement parser and MIR lowering**

Add grammar, AST, parser consumption, MIR function model, and transform lowering for function signatures.

- [x] **Step 4: Run focused and workspace tests**

Run:
```bash
cargo test -p fkl_parser function_signatures_lower_to_mir
cargo test --all
```

Expected: PASS.

- [x] **Step 5: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-function-support.md fkl_parser/src/parser/fkl.pest fkl_parser/src/parser/ast.rs fkl_parser/src/parser/parser.rs fkl_mir/src/strategy/context_map.rs fkl_parser/src/tests.rs fkl_parser/src/transform.rs README.md
git commit -m "feat(language): add function signatures"
git push origin master
```
