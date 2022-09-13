use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(super) fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl Geometry for #ident {}
    };

    output.into()
}
