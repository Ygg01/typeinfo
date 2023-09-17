extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate typeinfo_core;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, LifetimeParam};

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
    let lifetimes = get_lifetimes(generics.lifetimes());
    // TODO get nested generics

    let inner = match ast.data {
        Data::Struct(ref struct_data) => get_fields(&struct_data.fields),
        _ => panic!("Only Structs are supported. Enums/Unions cannot be turned into Generics."),
    };
    quote! {
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics #name #ty_generics #where_clause{

            const fn typeinfo() -> ::typeinfo_core::TypeInfo {
                use core::alloc::Layout;
                ::typeinfo_core::TypeInfo {
                    name: &#ident_str,
                    inner:  ::typeinfo_core::InnerTypeInfo::None,
                    layout: Layout::new::<#name #ty_generics>(),
                    generics: &[],
                    lifetimes: #lifetimes,
                }

            }
        }
    }
}

fn get_lifetimes<'a>(lifetimes: impl Iterator<Item = &'a LifetimeParam>) -> impl ToTokens {
    let mut start = TokenStream2::default();

    let lifetimes: Vec<_> = lifetimes
        .map(|type_param| {
            let lifetime_str = type_param.lifetime.ident.to_string();
            quote! {
                 ::typeinfo_core::LifetimeInfo {
                    name: #lifetime_str
                },
            }
        })
        .collect();
    start.extend(quote! { &[#(#lifetimes)*] });
    start
}

fn get_fields(data: &Fields) -> impl ToTokens {
    let mut start = TokenStream2::default();

    // let mut fields : Vec<Field> = vec![];
    // for field in data.fields {
    //
    // }
    // let fields = fields.leak();
    // StructInfo {
    //     fields,
    // }
    start
}
