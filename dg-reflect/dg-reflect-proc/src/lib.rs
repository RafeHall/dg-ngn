use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};



#[proc_macro_derive(Reflect)]
pub fn reflect(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics dg_reflect::Reflect for #name #ty_generics #where_clause {
            
        }
    };

    TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(dg_reflect::Reflect));
        }
    }
    generics
}