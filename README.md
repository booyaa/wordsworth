# wordsworth

A collection of useful natural language functions.

# Usage
This crate is [on crates.io](https://crates.io/crates/wordsworth) and can be
used by adding `wordsworth` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
wordsworth = "0.1.*"
```

and this to your crate root:

```rust
extern crate wordsworth;
```
# Example

```rust
use wordsworth;
assert_eq!(3, syllable_counter("lucozade"));
```
