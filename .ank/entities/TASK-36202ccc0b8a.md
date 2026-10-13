---
id: TASK-36202ccc0b8a
type: task
slug: hub-git-smart-http-v2-front-serving-refs-ank-and
title: "hub-git: smart HTTP front serving refs/ank/* (ls-refs and fetch over protocol v2, push over receive-pack) and refusing every other ref"
created: 2026-10-09T13:16:16Z
author: claude-code/fable-5.1
status: open
scope:
  - crates/hub-git/**
  - Cargo.toml
blocked_by: [TASK-b6c837a0eb63]
done_criteria: |
  cargo test --workspace passes with an integration test in crates/hub-git that starts the front over hub-store on a local port, points the installed ank 0.9 at it as origin with default_branch configured, and runs claim, release, done and completion end to end with the refs landing in the store; a test that a push to refs/heads/* or refs/tags/* is refused with an ng line whose reason is namespace, visible in git's output; and a test that after a refused push the same client's ls-remote of refs/ank/* still answers.
criteria_by: creator
verify: [cargo-test]
schema: 4
version: 5
---

Two protocols, because git has two. Protocol v2 defines `ls-refs`, `fetch`,
`server-option`, `object-format`, `object-info` and `bundle-uri`, and no push
(gitprotocol-v2, git 2.54): `git push` always speaks receive-pack over
protocol v0 with `report-status`, whatever `protocol.version` says. So the
front serves `ls-refs` and `fetch` for `refs/ank/*` over v2, and accepts
pushes through `GET /info/refs?service=git-receive-pack` and
`POST /git-receive-pack` over v0, mapping the command list onto the store's
compare-and-swap, atomic across the list.

ank 0.9 pushes one ref per push with `--force-with-lease=<ref>:<expected>`,
so a push carries one command with an explicit old oid; the empty expected
value means "must not exist" and is honoured as such.

A refusal is an `ng <ref> <reason>` line in report-status with the reason
tokens the amending ADR of ADR-38da5a578318 names (`held`, `namespace` in
phase 1), and `ls-refs` and `fetch` keep answering the client whose push was
refused: that is what keeps ank from reading a refusal as an unreachable
remote.

The front never sees `.ank/` files; it unpacks only blobs and refuses any
other object kind.
