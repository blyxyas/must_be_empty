# Must be empty

This attribute checks that a given function is empty. It is just that.
This can be useful for debugging, when you're just using using `cargo test` and want to assure that nobody

## Example

```rust
use must_be_empty::must_be_empty;

#[must_be_empty]
fn main() {}
```

If you were to put something in that `main` function, it would give a warning or an error, depending on [your configuration](#features).

## Installation

Put this in your `Cargo.toml` file:

```toml
[dependencies]
must_be_empty = "0.1.0"
```

## Features

* `warn` (*Default*): Warns, instead of outputing a hard-error.
* `only_on_release`: Only works in release mode (`--release`)
* `only_on_debug`: Only works in debug mode.