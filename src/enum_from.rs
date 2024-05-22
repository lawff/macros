use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
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
}
