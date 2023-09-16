extern crate typeinfo_core;
// extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data};
use typeinfo_core::Reflect;
use quote::{quote, ToTokens};


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

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    #[allow(clippy::let_and_return)]
    match ast.data {
        Data::Struct(ref _data) => {

        }
        _ => panic!("Only Structs are supported. Enums/Unions cannot be turned into Generics."),
    };
    quote! {
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics #name #ty_generics{

            const fn typeinfo() -> ::typeinfo_core::Type {
                use core::alloc::Layout;
                ::typeinfo_core::Type {
                    name: &#ident_str,
                    inner:  ::typeinfo_core::TypeInner::None,
                    layout: Layout::new::<#name #ty_generics>(),
                    generics: &[],
                    lifetimes: &[],
                }

            }
        }
    }

}
