---
id: TASK-1b0589c74feb
type: task
slug: hub-tenancy-and-hub-auth-organisations-corpora-r
title: "hub-tenancy and hub-auth: organisations, corpora registered with a root commit, bearer tokens and SSH keys"
created: 2026-10-09T13:16:18Z
author: claude-code/fable-5.1
status: open
scope:
  - crates/hub-tenancy/**
  - crates/hub-auth/**
  - Cargo.toml
blocked_by: [TASK-b6c837a0eb63]
done_criteria: |
  cargo test --workspace passes with tests proving: a corpus is created under an organisation at <org>/<corpus> with its root commit recorded; a registration naming a root commit another corpus of the same organisation carries is refused; an HTTP request without a token of the corpus's organisation is refused with 401 and one with another organisation's token with 403; an SSH key registered for an organisation reaches only that organisation's corpora.
criteria_by: creator
verify: [cargo-test]
schema: 4
version: 2
---

Minimal auth: enough to run a private hub. Tokens are per organisation, not
per identity; binding a token to an ank identity and attenuating it by scope
is phase 2 and waits on the ephemeral identity research track.
