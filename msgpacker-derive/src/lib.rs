#![crate_type = "proc-macro"]
extern crate proc_macro;

// This code is bad and should be refactored into something cleaner. Maybe some syn-based
// framework?

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, parse_quote, parse_str, Block, Data, DataEnum, DataStruct, DataUnion,
    DeriveInput, Expr, ExprMatch, ExprTuple, Field, FieldPat, FieldValue, Fields, FieldsNamed,
    FieldsUnnamed, GenericArgument, Ident, Member, Meta, Pat, PatIdent, PathArguments, Token, Type,
    Variant,
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

fn impl_fields_named(name: Ident, f: FieldsNamed) -> impl Into<TokenStream> {
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

    let (
        mut block_packable,
        mut block_unpackable,
        mut block_unpackable_iter
    ) = f.named.into_pairs().map(|p| p.into_value()).fold(
            (block_packable, block_unpackable, block_unpackable_iter),
            |(mut block_packable, mut block_unpackable, mut block_unpackable_iter), field| {
                let ident = field.ident.as_ref().cloned().unwrap();
                let ty = field.ty.clone();

                let mut is_vec = false;
                let mut is_vec_u8 = false;

                match &ty {
                    Type::Path(p) if p.path.segments.last().filter(|p| p.ident.to_string() == "Vec").is_some() => {
                        is_vec = true;
                        match &p.path.segments.last().unwrap().arguments {
                            PathArguments::AngleBracketed(a) if a.args.len() == 1 => {
                                if let Some(GenericArgument::Type(Type::Path(p))) = a.args.first() {
                                    if p.path.segments.last().filter(|p| p.ident.to_string() == "u8").is_some() {
                                        is_vec_u8 = true;
                                    }
                                }
                            }
                            _ => (),
                        }
                    }

                    _ => (),
                }

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
                } else if contains_attribute(&field, "array") || is_vec && !is_vec_u8 {
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
        );

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

    quote! {
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
    }
}

fn impl_fields_unnamed(name: Ident, f: FieldsUnnamed) -> impl Into<TokenStream> {
    let mut values: Punctuated<Expr, Token![,]> = Punctuated::new();
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

    let (mut block_packable, mut block_unpackable, mut block_unpackable_iter) = f
        .unnamed
        .into_pairs()
        .map(|p| p.into_value())
        .enumerate()
        .fold(
            (block_packable, block_unpackable, block_unpackable_iter),
            |(mut block_packable, mut block_unpackable, mut block_unpackable_iter), (i, field)| {
                let ty = field.ty.clone();
                let var: Expr = parse_str(format!("v{}", i).as_str()).unwrap();
                let slf: Expr = parse_str(format!("self.{}", i).as_str()).unwrap();

                if contains_attribute(&field, "map") {
                    todo!("unnamed map is not implemented for derive macro; implement the traits manually")
                } else if contains_attribute(&field, "array") {
                    todo!("unnamed array is not implemented for derive macro; implement the traits manually")
                } else {
                    block_packable.stmts.push(parse_quote! {
                        n += <#ty as ::msgpacker::Packable>::pack(&#slf, buf);
                    });

                    block_unpackable.stmts.push(parse_quote! {
                        let #var = ::msgpacker::Unpackable::unpack(buf).map(|(nv, t)| {
                            n += nv;
                            buf = &buf[nv..];
                            t
                        })?;
                    });

                    block_unpackable_iter.stmts.push(parse_quote! {
                        let #var = ::msgpacker::Unpackable::unpack_iter(bytes.by_ref()).map(|(nv, t)| {
                            n += nv;
                            t
                        })?;
                    });
                }

                values.push(var);

                (block_packable, block_unpackable, block_unpackable_iter)
            },
        );

    block_packable.stmts.push(parse_quote! {
        return n;
    });

    block_unpackable.stmts.push(parse_quote! {
        return Ok((n, Self(#values)));
    });

    block_unpackable_iter.stmts.push(parse_quote! {
        return Ok((n, Self(#values)));
    });

    quote! {
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
    }
}

fn impl_fields_unit(name: Ident) -> impl Into<TokenStream> {
    quote! {
        impl ::msgpacker::Packable for #name {
            fn pack<T>(&self, _buf: &mut T) -> usize
            where
                T: Extend<u8>,
            {
                0
            }
        }

        impl ::msgpacker::Unpackable for #name {
            type Error = ::msgpacker::Error;

            fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
                Ok((0, Self))
            }

            fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
            where
                I: IntoIterator<Item = u8>,
            {
                Ok((0, Self))
            }
        }
    }
}

fn impl_fields_enum(name: Ident, v: Punctuated<Variant, Token![,]>) -> impl Into<TokenStream> {
    if v.is_empty() {
        todo!("empty enum is not implemented for derive macro; implement the traits manually");
    }

    let mut block_packable: ExprMatch = parse_quote! {
        match self {
        }
    };

    let mut block_unpackable: ExprMatch = parse_quote! {
        match discriminant {
        }
    };

    let mut block_unpackable_iter: ExprMatch = parse_quote! {
        match discriminant {
        }
    };

    v.into_iter().enumerate().for_each(|(i, v)| {
        let discriminant = v
            .discriminant
            .map(|(_, d)| d)
            .unwrap_or_else(|| parse_str(format!("{}", i).as_str()).unwrap());

        // TODO check attributes of the field
        let ident = v.ident.clone();
        match v.fields {
            Fields::Named(f) => {
                let mut blk: Block = parse_str("{}").unwrap();
                let mut blk_unpack: Block = parse_str("{}").unwrap();
                let mut blk_unpack_iter: Block = parse_str("{}").unwrap();
                let mut blk_unpack_fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();

                blk.stmts.push(parse_quote! {
                    n += (#discriminant as u32).pack(buf);
                });

                f.named
                    .iter()
                    .filter_map(|n| n.ident.as_ref())
                    .for_each(|field| {
                        blk.stmts.push(parse_quote! {
                            n += #field.pack(buf);
                        });

                        blk_unpack_fields.push(parse_quote! { #field });

                        blk_unpack.stmts.push(parse_quote! {
                            let #field = Unpackable::unpack(buf).map(|(nv, t)| {
                                n += nv;
                                buf = &buf[nv..];
                                t
                            })?;
                        });

                        blk_unpack_iter.stmts.push(parse_quote! {
                            let #field = Unpackable::unpack_iter(bytes.by_ref()).map(|(nv, t)| {
                                n += nv;
                                t
                            })?;
                        });
                    });

                let mut arm: syn::Arm = parse_quote! {
                    #name::#ident {} => #blk,
                };

                f.named
                    .iter()
                    .filter_map(|n| n.ident.as_ref())
                    .for_each(|field| {
                        match &mut arm.pat {
                            Pat::Struct(s) => {
                                s.fields.push(FieldPat {
                                    attrs: vec![],
                                    member: Member::Named(field.clone()),
                                    colon_token: None,
                                    pat: Box::new(Pat::Ident(PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: field.clone(),
                                        subpat: None,
                                    })),
                                });
                            }
                            _ => todo!(
                                "enum variant is not implemented for derive macro; implement the traits manually"
                            ),
                        }
                    });

                block_packable.arms.push(arm);

                blk_unpack.stmts.push(parse_quote! {
                    slf = #name::#ident { #blk_unpack_fields };
                });

                blk_unpack_iter.stmts.push(parse_quote! {
                    slf = #name::#ident { #blk_unpack_fields };
                });

                block_unpackable.arms.push(parse_quote! {
                    #discriminant => #blk_unpack,
                });

                block_unpackable_iter.arms.push(parse_quote! {
                    #discriminant => #blk_unpack_iter,
                });
            }

            Fields::Unnamed(f) => {
                let mut blk: Block = parse_str("{}").unwrap();
                let mut blk_unpack: Block = parse_str("{}").unwrap();
                let mut blk_unpack_iter: Block = parse_str("{}").unwrap();

                blk.stmts.push(parse_quote! {
                    n += (#discriminant as u32).pack(buf);
                });

                let mut tuple_arm: ExprTuple = parse_str("()").unwrap();
                f.unnamed.iter().enumerate().for_each(|(ii, _field)| {
                    let ti: Expr = parse_str(format!("t{}", ii).as_str()).unwrap();
                    tuple_arm.elems.push(ti.clone());

                    blk.stmts.push(parse_quote! {
                        n += #ti.pack(buf);
                    });

                    blk_unpack.stmts.push(parse_quote! {
                        let #ti = Unpackable::unpack(buf).map(|(nv, t)| {
                            n += nv;
                            buf = &buf[nv..];
                            t
                        })?;
                    });

                    blk_unpack_iter.stmts.push(parse_quote! {
                        let #ti = Unpackable::unpack_iter(bytes.by_ref()).map(|(nv, t)| {
                            n += nv;
                            t
                        })?;
                    });
                });

                blk_unpack.stmts.push(parse_quote! {
                    slf = #name::#ident #tuple_arm;
                });

                blk_unpack_iter.stmts.push(parse_quote! {
                    slf = #name::#ident #tuple_arm;
                });

                block_packable.arms.push(parse_quote! {
                    #name::#ident #tuple_arm => #blk,
                });

                block_unpackable.arms.push(parse_quote! {
                    #discriminant => #blk_unpack,
                });

                block_unpackable_iter.arms.push(parse_quote! {
                    #discriminant => #blk_unpack_iter,
                });
            }

            Fields::Unit => {
                block_packable.arms.push(parse_quote! {
                    #name::#ident => {
                        n += (#discriminant as u32).pack(buf);
                    }
                });

                block_unpackable.arms.push(parse_quote! {
                    #discriminant => slf = #name::#ident,
                });

                block_unpackable_iter.arms.push(parse_quote! {
                    #discriminant => slf = #name::#ident,
                });
            }
        }
    });

    block_unpackable.arms.push(parse_quote! {
        _ => {
            return Err(::msgpacker::Error::InvalidEnumVariant);
        }
    });

    block_unpackable_iter.arms.push(parse_quote! {
        _ => {
            return Err(::msgpacker::Error::InvalidEnumVariant);
        }
    });

    quote! {
        impl ::msgpacker::Packable for #name {
            fn pack<T>(&self, buf: &mut T) -> usize
            where
                T: Extend<u8>,
            {
                let mut n = 0;

                #block_packable;

                return n;
            }
        }

        impl ::msgpacker::Unpackable for #name {
            type Error = ::msgpacker::Error;

            #[allow(unused_mut)]
            fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
                let (mut n, discriminant) = u32::unpack(&mut buf)?;
                buf = &buf[n..];
                let slf;

                #block_unpackable;

                Ok((n, slf))
            }

            fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
            where
                I: IntoIterator<Item = u8>,
            {
                let mut bytes = bytes.into_iter();
                let (mut n, discriminant) = u32::unpack_iter(bytes.by_ref())?;
                let slf;

                #block_unpackable_iter;

                Ok((n, slf))
            }
        }
    }
}

#[proc_macro_derive(MsgPacker, attributes(msgpacker))]
pub fn msg_packer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;
    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(f),
            ..
        }) => impl_fields_named(name, f).into(),

        Data::Struct(DataStruct {
            fields: Fields::Unnamed(f),
            ..
        }) => impl_fields_unnamed(name, f).into(),

        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => impl_fields_unit(name).into(),

        Data::Enum(DataEnum { variants, .. }) => impl_fields_enum(name, variants).into(),

        Data::Union(DataUnion { .. }) => {
            todo!(
                "union support is not implemented for derive macro; implement the traits manually"
            )
        }
    }
}
