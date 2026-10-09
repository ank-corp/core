---
id: TASK-36202ccc0b8a
type: task
slug: hub-git-smart-http-v2-front-serving-refs-ank-and
title: "hub-git: smart HTTP v2 front serving refs/ank/* and refusing every other ref"
created: 2026-10-09T13:16:16Z
author: claude-code/fable-5.1
status: open
scope:
  - crates/hub-git/**
  - Cargo.toml
blocked_by: [TASK-b6c837a0eb63]
done_criteria: |
  cargo test --workspace passes with an integration test in crates/hub-git that starts the front over hub-store on a local port, points the installed ank 0.9 at it as origin, and runs claim, release, done and completion end to end with the refs landing in the store; and a test that a push to refs/heads/* or refs/tags/* is refused with the refusal named in git's output.
criteria_by: creator
verify: [cargo-test]
schema: 4
version: 2
---

Protocol v2 only: ls-refs and fetch for `refs/ank/*`, receive-pack with
atomic multi-ref updates mapped onto the store's CAS. The front never sees
`.ank/` files; it unpacks only blobs and refuses any other object kind.
