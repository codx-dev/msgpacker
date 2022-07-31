#![crate_type = "proc-macro"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, parse_quote, Block, Data, DeriveInput, FieldValue, Fields, Member, Token,
};

#[proc_macro_derive(MsgPacker)]
pub fn msg_packer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut values: Punctuated<FieldValue, Token![,]> = Punctuated::new();
    let (block, block_size): (Block, Block) = match data {
        Data::Struct(syn::DataStruct {
            struct_token: _,
            fields: Fields::Named(f),
            semi_token: _,
        }) => f
            .named
            .into_pairs()
            .map(|p| p.into_value())
            .fold((syn::parse_str("{}").unwrap(), syn::parse_str("{}").unwrap()), |(mut block, mut block_size), field| {
                let ident = field.ident.as_ref().cloned().unwrap();
                let ty = field.ty;

                block_size.stmts.push(parse_quote! {
                    n += <#ty as msgpacker::prelude::SizeableMessage>::packed_len(&self.#ident);
                });

                block.stmts.push(parse_quote! {
                    n += <#ty as msgpacker::prelude::Packable>::pack(&self.#ident, packer.by_ref())?;
                });

                let fv = FieldValue {
                    attrs: vec![],
                    member: Member::Named(ident),
                    colon_token: Some(<Token![:]>::default()),
                    expr: parse_quote! {
                        <#ty as msgpacker::prelude::Unpackable>::unpack(unpacker.by_ref())?
                    },
                };
                values.push(fv);

                (block, block_size)
            }),
        _ => todo!(),
    };

    let expanded = quote! {
        impl msgpacker::prelude::SizeableMessage for #name {
            fn packed_len(&self) -> usize {
                let mut n = 0;

                #block_size

                n
            }
        }

        impl msgpacker::prelude::Packable for #name {
            fn pack<W>(&self, mut packer: W) -> std::io::Result<usize>
            where
                W: std::io::Write
            {
                let mut n = 0;

                #block

                Ok(n)
            }
        }

        impl msgpacker::prelude::Unpackable for #name {
            fn unpack<R>(mut unpacker: R) -> std::io::Result<Self>
            where
                R: std::io::BufRead,
            {
                Ok(Self {
                    #values
                })
            }
        }
    };

    TokenStream::from(expanded)
}
