---
id: TASK-d0afbd95f637
type: task
slug: core-has-a-ci-that-runs-the-workspace-tests-on-l
title: core has a CI that runs the workspace tests on Linux, macOS and Windows for every push and pull request to main
created: 2026-10-13T08:18:47Z
author: claude-code/fable-5.1
status: open
scope:
  - .github/workflows/**
blocked_by: []
done_criteria: |
  .github/workflows/ci.yml exists, triggers on push and on pull_request targeting main, runs cargo test --workspace on ubuntu-latest, macos-latest and windows-latest with the stable toolchain and a cargo cache, and the workflow run for the commit that closes this task is green on all three; the proof names that run.
criteria_by: creator
verify: [cargo-test]
schema: 4
version: 1
---

A project whose thesis is "nobody declares themselves done" cannot have its
own proofs run only on the agent's machine. This is the minimum: the
workspace's tests on the three platforms ank supports.

It is also the first testbed for the decision that an independent
attestation comes from the project's CI before any hub runner exists: once
the attestation path ships, this workflow is where core's own proofs get
attested. Nothing of that is in this task; keep the workflow minimal and
boring. No release workflow yet.
