//! The ref store. A corpus's `refs/ank/*` and the blobs they name, updated by
//! compare-and-swap with git `receive-pack` semantics. Implementation lands
//! with its task.

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
