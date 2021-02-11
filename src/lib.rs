//! Adds the yolo keyword. `#[yolo]` is equivalent to `unsafe`.

use proc_macro::{Ident, Span, TokenStream, TokenTree};
use std::iter::once;

/// See crate docs for details.
#[proc_macro_attribute]
pub fn yolo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    once(TokenTree::Ident(Ident::new("unsafe", Span::call_site())))
        .chain(item)
        .collect()
}
