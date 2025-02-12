use super::utils::gen_param_input;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    let mut is_tuple_struct = false;
    match &item.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = data.fields {
                is_tuple_struct = true
            }
        }
        _ => todo!("Currently only structs are supported"),
    }

    let field_names = || struct_fields(&item).map(|field| &field.ident);
    let field_types = || struct_fields(&item).map(|field| &field.ty);
    let seq =
        || (0..field_names().count()).map(|i| TokenStream2::from_str(&i.to_string()).unwrap());

    let name = &item.ident;
    let generics = &item.generics;
    let where_clause = &generics.where_clause;
    let generic_params = gen_param_input(&generics, true);

    let field_types_encoding = field_types();
    let seq_substore = seq();
    let seq_data = seq();

    let create_body = if is_tuple_struct {
        quote!(
            Ok(Self(
                #(
                    ::orga::state::State::create(
                        store.sub(&[#seq_substore]),
                        data.#seq_data,
                    )?,
                )*
            ))
        )
    } else {
        let names = field_names();
        quote!(
            Ok(Self {
                #(
                    #names: ::orga::state::State::create(
                        store.sub(&[#seq_substore]),
                        data.#seq_data,
                    )?,
                )*
            })
        )
    };

    let flush_body = if is_tuple_struct {
        let indexes = seq();
        quote!(
            Ok((
                #(::orga::state::State::<::orga::store::DefaultBackingStore>::flush(self.#indexes)?,)*
            ))
        )
    } else {
        let names = field_names();
        quote!(
            Ok((

                #(::orga::state::State::<::orga::store::DefaultBackingStore>::flush(self.#names)?,)*
            ))
        )
    };

    let from_body = if is_tuple_struct {
        let indexes = seq();
        quote! (
            #(value.#indexes.into(),)*
        )
    } else {
        let names = field_names();
        quote! (
            #(value.#names.into(),)*
        )
    };

    let output = quote! {
        impl#generics ::orga::state::State for #name#generic_params
        #where_clause
        {
            type Encoding = (
                #(
                    <#field_types_encoding as ::orga::state::State>::Encoding,
                )*
            );

            fn create(
                store: ::orga::store::Store,
                data: Self::Encoding,
            ) -> ::orga::Result<Self> {
                #create_body
            }

            fn flush(self) -> ::orga::Result<Self::Encoding> {
                #flush_body
            }
        }

        impl#generics From<#name#generic_params> for <#name#generic_params as ::orga::state::State>::Encoding
        #where_clause
        {
            fn from(value: #name#generic_params) -> Self {
                (#from_body)
            }
        }
    };

    output.into()
}

fn struct_fields(item: &DeriveInput) -> impl Iterator<Item = &Field> {
    let data = match item.data {
        Data::Struct(ref data) => data,
        Data::Enum(ref _data) => todo!("#[derive(State)] does not yet support enums"),
        Data::Union(_) => panic!("Unions are not supported"),
    };

    match data.fields {
        Fields::Named(ref fields) => fields.named.iter(),
        Fields::Unnamed(ref fields) => fields.unnamed.iter(),
        Fields::Unit => panic!("Unit structs are not supported"),
    }
}
