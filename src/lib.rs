//! # Must be empty
//! 
//! This attribute checks that a given function is empty. It is just that.
//! This can be useful for debugging, when you're just using using `cargo test` and want to assure that nobody
//! 
//! ## Example
//! 
//! ```rust
//! use must_be_empty::must_be_empty;
//! 
//! #[must_be_empty]
//! fn main() {}
//! ```
//! 
//! ## Installation
//! 
//! Put this in your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! must_be_empty = "0.1.0"
//! ```

use proc_macro::TokenStream;
use proc_macro_error::{emit_error, emit_warning, proc_macro_error};
use quote::quote;

use syn::{parse_macro_input, ItemFn};

#[proc_macro_error]
#[proc_macro_attribute]
/// Makes sure that a specific function is empty.
///
/// # Example
///
/// This will not cause any problems
pub fn must_be_empty(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    if cfg!(feature = "only_on_release") {
        if !cfg!(debug_assertions) {
            visit(input)
        } else {
            input
        }
    } else if cfg!(feature = "only_on_debug") {
		if cfg!(debug_assertions) {
			visit(input)
		} else {
			input
		}
	} else {
        visit(input)
    }
}

fn visit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    if input.block.stmts.len() >= 1 {
        if !cfg!(feature = "warn") {
            emit_error!(
                input.sig,
                format!(
                    "The function `{}` isn't empty. It has {} statements",
                    input.sig.ident,
                    input.block.stmts.len()
                );
                help = "Try removing the contents of this function."
            );
        } else {
            emit_warning!(
                input.sig,
                format!(
                    "The function `{}` isn't empty. It has {} statements",
                    input.sig.ident,
                    input.block.stmts.len()
                );
                help = "Try removing the contents of this function."
            );
        }
    }
    TokenStream::from(quote!(#input))
}
