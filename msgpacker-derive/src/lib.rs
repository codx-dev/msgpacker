#![crate_type = "proc-macro"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, parse_quote, Block, Data, DataStruct, DeriveInput, Field, FieldValue,
    Fields, Member, Meta, Token,
};

fn contains_attribute(field: &Field, name: &str) -> bool {
    let name = name.to_string();
    if let Some(attr) = field.attrs.first() {
        if let Meta::List(list) = &attr.meta {
            if list.path.is_ident("msgpacker") {
                if list
                    .tokens
                    .clone()
                    .into_iter()
                    .find(|a| a.to_string() == name)
                    .is_some()
                {
                    return true;
                }
            }
        }
    }
    false
}

#[proc_macro_derive(MsgPacker, attributes(msgpacker))]
pub fn msg_packer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut values: Punctuated<FieldValue, Token![,]> = Punctuated::new();
    let block_packable: Block = parse_quote! {
        {
            let mut n = 0;
        }
    };
    let block_unpackable: Block = parse_quote! {
        {
            let mut n = 0;
        }
    };
    let block_unpackable_iter: Block = parse_quote! {
        {
            let mut bytes = bytes.into_iter();
            let mut n = 0;
        }
    };

    let (mut block_packable, mut block_unpackable, mut block_unpackable_iter): (
        Block,
        Block,
        Block,
    ) = match data {
        Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Named(f),
            semi_token: _,
        }) => f.named.into_pairs().map(|p| p.into_value()).fold(
            (block_packable, block_unpackable, block_unpackable_iter),
            |(mut block_packable, mut block_unpackable, mut block_unpackable_iter), field| {
                let ident = field.ident.as_ref().cloned().unwrap();
                let ty = field.ty.clone();

                if contains_attribute(&field, "map") {
                    block_packable.stmts.push(parse_quote! {
                        n += ::msgpacker::pack_map(buf, &self.#ident);
                    });

                    block_unpackable.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::unpack_map(buf).map(|(nv, t)| {
                            n += nv;
                            buf = &buf[nv..];
                            t
                        })?;
                    });

                    block_unpackable_iter.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::unpack_map_iter(bytes.by_ref()).map(|(nv, t)| {
                            n += nv;
                            t
                        })?;
                    });
                } else if contains_attribute(&field, "array") {
                    block_packable.stmts.push(parse_quote! {
                        n += ::msgpacker::pack_array(buf, &self.#ident);
                    });

                    block_unpackable.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::unpack_array(buf).map(|(nv, t)| {
                            n += nv;
                            buf = &buf[nv..];
                            t
                        })?;
                    });

                    block_unpackable_iter.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::unpack_array_iter(bytes.by_ref()).map(|(nv, t)| {
                            n += nv;
                            t
                        })?;
                    });
                } else {
                    block_packable.stmts.push(parse_quote! {
                        n += <#ty as ::msgpacker::Packable>::pack(&self.#ident, buf);
                    });

                    block_unpackable.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::Unpackable::unpack(buf).map(|(nv, t)| {
                            n += nv;
                            buf = &buf[nv..];
                            t
                        })?;
                    });

                    block_unpackable_iter.stmts.push(parse_quote! {
                        let #ident = ::msgpacker::Unpackable::unpack_iter(bytes.by_ref()).map(|(nv, t)| {
                            n += nv;
                            t
                        })?;
                    });
                }


                values.push(FieldValue {
                    attrs: vec![],
                    member: Member::Named(ident.clone()),
                    colon_token: Some(<Token![:]>::default()),
                    expr: parse_quote! { #ident },
                });

                (block_packable, block_unpackable, block_unpackable_iter)
            },
        ),
        _ => todo!(),
    };

    block_packable.stmts.push(parse_quote! {
        return n;
    });

    block_unpackable.stmts.push(parse_quote! {
        return Ok((
            n,
            Self {
                #values
            },
        ));
    });

    block_unpackable_iter.stmts.push(parse_quote! {
        return Ok((
            n,
            Self {
                #values
            },
        ));
    });

    let expanded = quote! {
        impl ::msgpacker::Packable for #name {
            fn pack<T>(&self, buf: &mut T) -> usize
            where
                T: Extend<u8>,
                #block_packable
        }

        impl ::msgpacker::Unpackable for #name {
            type Error = ::msgpacker::Error;

            fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error>
                #block_unpackable

            fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
            where
                I: IntoIterator<Item = u8>,
                #block_unpackable_iter
        }
    };

    TokenStream::from(expanded)
}
