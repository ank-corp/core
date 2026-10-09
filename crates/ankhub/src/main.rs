//! The hub binary. Composition of the crates lands with the tasks that build them.

fn main() {
    println!("ankhub {} (hub-store {})", env!("CARGO_PKG_VERSION"), hub_store::VERSION);
}
