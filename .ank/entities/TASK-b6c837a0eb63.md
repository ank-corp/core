---
id: TASK-b6c837a0eb63
type: task
slug: hub-store-atomic-compare-and-swap-on-refs-ank-wi
title: "hub-store: atomic compare-and-swap on refs/ank/* with a write-ahead log and per-corpus sequence numbers"
created: 2026-10-09T13:16:16Z
author: claude-code/fable-5.1
status: open
scope:
  - crates/hub-store/**
blocked_by: []
done_criteria: |
  cargo test --workspace passes with tests in crates/hub-store proving: a multi-ref update succeeds only if every stored value equals its old and is applied all-or-none; a ref outside refs/ank/* or an object that is not a blob is refused; each successful update receives a strictly increasing per-corpus sequence number and a receive timestamp readable afterwards; blobs are stored content-addressed and read back byte-identical; a store process killed after an update was acknowledged shows that update after recovery, and one killed before acknowledgement shows nothing of it.
criteria_by: creator
verify: [cargo-test]
schema: 4
version: 1
---

SQLite in WAL mode for refs, sequence and metadata; content-addressed files
for blobs. The durability test spawns the store as a child process with a
crash point and reopens the data directory.

This is the first piece of git infrastructure AnkHub owns; it is built
regardless of phase 0 results.
