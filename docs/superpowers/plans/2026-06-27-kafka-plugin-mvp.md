# Kafka Plugin MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the `ext_kafka` plugin a testable MVP with a stable runner command surface and environment-derived broker configuration.

**Architecture:** Keep the plugin boundary inside `extensions/ext_kafka`. Add pure configuration helpers around the existing `KafkaRunner` so behavior can be verified without requiring a live Kafka broker.

**Tech Stack:** Rust workspace crate `fkl_ext_kafka`, `fkl_ext_api::CustomRunner`, `fkl_mir::CustomEnv`, `cargo test`.

---

### Task 1: Kafka Runner Command Surface

**Files:**
- Modify: `extensions/ext_kafka/src/lib.rs`
- Modify: `extensions/ext_kafka/src/kafka_runner.rs`

- [x] **Step 1: Write the failing tests**

Add tests that assert `KafkaExt` exposes the `broker` command, reports its stable name as `kafka`, and formats the broker from `host` and `port` attributes.

- [x] **Step 2: Run tests to verify they fail**

Run: `cargo test -p fkl_ext_kafka`

Expected: FAIL because `KafkaExt::name()` still returns `KafkaRunner`, `list_commands()` is empty, and `send_command("broker", ...)` is not implemented.

- [x] **Step 3: Write minimal implementation**

Expose `KafkaRunner::brokers()`, avoid panics for missing malformed env attrs, return command list `["broker"]`, and implement `send_command("broker", ...)`.

- [x] **Step 4: Run focused tests**

Run: `cargo test -p fkl_ext_kafka`

Expected: PASS.

- [x] **Step 5: Run workspace tests**

Run: `cargo test --all`

Expected: PASS.

- [ ] **Step 6: Commit and push**

```bash
git add docs/superpowers/plans/2026-06-27-kafka-plugin-mvp.md extensions/ext_kafka/src/lib.rs extensions/ext_kafka/src/kafka_runner.rs README.md
git commit -m "feat(kafka): expose plugin broker command"
git push origin master
```
