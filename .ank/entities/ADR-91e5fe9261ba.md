---
id: ADR-91e5fe9261ba
type: adr
slug: claim-objects-are-opaque-to-the-hub-and-core-dep
title: Claim objects are opaque to the hub, and core depends on no ank crate until a decision names the need
created: 2026-10-09T13:15:01Z
author: claude-code/fable-5.1
status: proposed
scope:
  - crates/**
  - Cargo.toml
constraint: |
  The store and the git front treat the objects refs/ank/* name as opaque bytes, and no crate in core depends on a crate from haksolot/ank.
schema: 4
version: 1
---

Phase 1 needs no knowledge of ank: a compare-and-swap on refs whose objects are
bytes. The logic a later phase might want (the claim record format, expiry,
scope intersection) lives in `ank-cli` today, not in `ank-core`, so a dependency
"on ank-core pinned by tag" would buy nothing and tie the hub's release cadence
to ank's.

Hub-wide scope intersection, listed for phase 1 in the opportunity report, is
dropped as a hub feature: once every claim of a corpus lives on one hub and
`watch` mirrors them, the signal `claim` already emits is global. It needs task
scopes, which live in `.ank/`, which the hub does not hold.

When a phase genuinely needs ank's data model on the hub (identity binding,
phase 2), the need is named by an ADR that supersedes or amends this one, and
the code it needs is first extracted into `ank-core` by an ADR in
`haksolot/ank`.
