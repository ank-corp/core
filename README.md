# ank-corp/core

AnkHub is a remote, not a new arbiter. ank already coordinates agents through
git refs under `refs/ank/*`, updated by compare-and-swap. AnkHub speaks git for
that namespace, so an unmodified `ank` pointed at it works, and adds what a CLI
structurally cannot: a fast instrumented ref store, an event stream, and later
authenticated identities, verifier replay and integration.

This repository is the hub's foundation: a modular monolith in Rust whose
crates map onto future service boundaries, shipped as one self-hostable binary.

The plan and the decisions live in `.ank/`. Start with `ank context .`.

Licence: Apache-2.0.
