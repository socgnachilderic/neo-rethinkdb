use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(super) fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl Document for &#ident {
            fn get_document(&self) -> &Self {
                self
            }
        }

        impl Document for &Vec<#ident> {
            fn get_document(&self) -> &Self {
                self
            }
        }

        impl Document for &Vec<&#ident> {
            fn get_document(&self) -> &Self {
                self
            }
        }

        impl Document for &[#ident] {
            fn get_document(&self) -> &Self {
                self
            }
        }

        impl Document for &[&#ident] {
            fn get_document(&self) -> &Self {
                self
            }
        }
    };

    output.into()
}
