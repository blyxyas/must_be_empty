# Must be empty

This attribute checks that a given function is empty. It is just that.
This can be useful for debugging, when you're just using using `cargo test` and want to assure that nobody

## Example

```
use must_be_empty::must_be_empty;

#[must_be_empty]
fn main() {}
```

## Installation

Put this in your `Cargo.toml` file:

```toml
[dependencies]
must_be_empty = "0.1.0"
```