use proc_macro::TokenStream;
use proc_macro_error::{emit_error, emit_warning, proc_macro_error};
use quote::quote;

use syn::{parse_macro_input, ItemFn};

#[proc_macro_error]
#[proc_macro_attribute]
/// Makes sure that a specific function is empty. It only works in *release* mode. (Intended to do a `cargo check --release` before commiting)
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
