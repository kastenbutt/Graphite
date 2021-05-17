mod as_message;
mod combined_message_attrs;
mod discriminant;
mod helper_structs;
mod helpers;
//mod hint;
mod transitive_child;

use crate::as_message::derive_as_message_impl;
use crate::combined_message_attrs::combined_message_attrs_impl;
use crate::discriminant::derive_discriminant_impl;
use crate::transitive_child::derive_transitive_child_impl;
use proc_macro::TokenStream;

#[proc_macro_derive(
    ToDiscriminant,
    attributes(child, discriminant_derive, discriminant_attr)
)]
pub fn derive_discriminant(input_item: TokenStream) -> TokenStream {
    TokenStream::from(
        derive_discriminant_impl(input_item.into()).unwrap_or_else(|err| err.to_compile_error()),
    )
}

// todo: revert so that parent takes an expr as second arg again
#[proc_macro_derive(TransitiveChild, attributes(parent, parent_is_top))]
pub fn derive_transitive_child(input_item: TokenStream) -> TokenStream {
    TokenStream::from(
        derive_transitive_child_impl(input_item.into())
            .unwrap_or_else(|err| err.to_compile_error()),
    )
}

#[proc_macro_derive(AsMessage, attributes(child))]
pub fn derive_message(input_item: TokenStream) -> TokenStream {
    TokenStream::from(
        derive_as_message_impl(input_item.into()).unwrap_or_else(|err| err.to_compile_error()),
    )
}

#[proc_macro_attribute]
pub fn impl_message(attr: TokenStream, input_item: TokenStream) -> TokenStream {
    TokenStream::from(
        combined_message_attrs_impl(attr.into(), input_item.into())
            .unwrap_or_else(|err| err.to_compile_error()),
    )
}
