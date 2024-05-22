use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ident = input.ident;
    let generics = input.generics;
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Only single field variants are supported")
                } else {
                    let ty = &fields.unnamed.first().expect("Expected a field").ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(variant: #ty) -> Self {
                                Self::#variant_ident(variant)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_) => {
                quote! {}
            }
            syn::Fields::Unit => {
                quote! {}
            }
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
