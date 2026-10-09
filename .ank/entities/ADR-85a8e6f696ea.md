---
id: ADR-85a8e6f696ea
type: adr
slug: the-hub-s-receive-timestamp-and-sequence-number
title: The hub's receive timestamp and sequence number are the record of a coordination ref's transition
created: 2026-10-09T13:15:02Z
author: claude-code/fable-5.1
status: proposed
scope:
  - crates/**
constraint: |
  Every accepted update to refs/ank/* is assigned a monotonic per-corpus sequence number and a receive timestamp from the hub's clock, both exposed to readers.
schema: 4
version: 1
---

Read in ank 0.9.0: lease expiry is judged with the reader's local clock against
the `expires` field the claimant wrote, with a drift tolerance of two minutes.
Two machines offset by five minutes therefore disagree on which claims are
live. That answers the opportunity report's open question 1 without a
measurement: a single lease clock is a phase 1 requirement.

The hub cannot renew a lease and must not (renewal is by work, never by proxy),
but it can be the clock of record: each transition carries the instant the hub
accepted it and its position in the corpus's history. The ank side (computing
expiry from the receive timestamp when the coordination remote provides one) is
an ADR in `haksolot/ank`. The event stream carries both fields.
