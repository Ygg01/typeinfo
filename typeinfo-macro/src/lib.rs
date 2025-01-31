extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate typeinfo_core;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Generics, LifetimeParam, TypeParam};

use typeinfo_core::Reflect;

#[proc_macro_derive(Reflect)]
pub fn reflect_derive(ty: TokenStream) -> TokenStream {
    let tree = impl_generics(ty);
    tree.into_token_stream().into()
}

fn impl_generics(input: TokenStream) -> impl ToTokens {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let ident_str = name.to_string();
    let generics = &ast.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // let lifetimes = get_lifetimes(generics.lifetimes());
    let generics = get_generics(generics.type_params());

    let inner_type = match ast.data {
        Data::Struct(ref struct_data) => get_struct(&ident_str, &struct_data.fields, &generics),
        _ => panic!("Only Structs are supported. Enums/Unions cannot be turned into Generics."),
    };

    quote! {
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics #name #ty_generics #where_clause{

            const fn typeinfo(&self) -> ::typeinfo_core::TypeInfo {
                #inner_type
            }
        }
    }
}

fn get_generics<'a>(lifetimes: impl Iterator<Item=&'a TypeParam>) -> TokenStream2 {
    let mut start = TokenStream2::default();

    let lifetimes_str: Vec<_> = lifetimes
        .map(|generic_types| {
            let lifetime_str = generic_types.ident.to_string();

            quote! {
                ::typeinfo_core::LifetimeInfo{
                    name: #lifetime_str,
                }
            }
        })
        .collect();
    start.extend(quote! { &[#(#lifetimes_str)*] });
    start
}


fn get_lifetimes<'a>(lifetimes: impl Iterator<Item=&'a LifetimeParam>) -> TokenStream2 {
    let mut start = TokenStream2::default();

    let lifetimes: Vec<_> = lifetimes
        .map(|type_param| {
            let lifetime_str = type_param.lifetime.ident.to_string();
            quote! {
                ::typeinfo_core::LifetimeInfo {

                },
            }
        })
        .collect();
    start.extend(quote! { &[#(#lifetimes)*] });
    start
}

fn get_struct(name: &str, fields: &Fields, generics: &TokenStream2) -> impl ToTokens {
    let mut start = TokenStream2::default();

    start.extend(quote!(
        ::typeinfo_core::TypeInfo::Struct(::typeinfo_core::StructInfo {
            name: #name,
            lifetimes: #generics,
            fields: &[],
        })
    ));
    // let mut fields : Vec<Field> = vec![];
    for field in fields {}
    // let fields = fields.leak();
    // StructInfo {
    //     fields,
    // }
    start
}
