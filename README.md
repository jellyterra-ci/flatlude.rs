# flatlude.rs
Organize Rust symbols in module by directory, not by file.

# Usage

```toml
# Cargo.toml
[dependencies]
flatlude = "1.0.0"
```

```rust
// lib.rs | main.rs
flatlude::flatlude!();

// For example, the proc macro generates:

// for src/a.rs
pub mod a;
pub use a::*;

// for src/b/mod.rs
pub mod b;
```

```rust
// mod.rs
use crate::*;

fn example() {
    let _ =    A::new();
    let _ = b::B::new();
}
```

## rust-analyzer

Each time you create or remove modules from the source tree, you have to trigger rust-analyzer by editing and it will re-run the proc macro.

```rust
flatlude::flatlude!();
flatlude::flatlude!(edit);
flatlude::flatlude!(something);
flatlude::flatlude!(to);
flatlude::flatlude!(trigger);
```

## Licensing

Public domain.
