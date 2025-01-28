use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(HasSomeField, attributes(ignore_has_some))]
pub fn has_some_field_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = if let Data::Struct(data) = input.data {
        match data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|f| f.to_owned())
                .collect::<Vec<_>>(),
            _ => panic!("HasSomeField can only be derived for structs with named fields"),
        }
    } else {
        panic!("HasSomeField can only be derived for structs");
    };

    let ignore_fields: HashSet<*const Field> = fields
        .iter()
        .filter(|f| {
            f.attrs
                .iter()
                .any(|attr| attr.path().is_ident("ignore_has_some"))
        })
        .map(|f| f as *const Field)
        .collect();

    let fields: Vec<Ident> = fields
        .iter()
        .filter(|f| !ignore_fields.contains(&(*f as *const Field)))
        .map(|f| f.ident.to_owned().unwrap())
        .collect();

    let gen = quote! {
        impl #name {
            pub fn has_some_field(&self) -> bool {
                #(
                    if self.#fields.is_some() {
                        return true;
                    }
                )*
                false
            }

            pub fn some_field_count(&self) -> usize {
                let mut count = 0;
                #(
                    if self.#fields.is_some() {
                        count += 1;
                    }
                )*
                count
            }
        }
    };

    gen.into()
}
