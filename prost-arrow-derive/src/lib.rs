// This is my typical proc-macro prelude
#![allow(unused_imports)]
extern crate proc_macro;
use std::any::Any;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Result, *,
};

#[proc_macro_derive(ToArrow)]
pub fn rule_system_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as _);
    TokenStream::from(match impl_my_trait(ast) {
        Ok(it) => it,
        Err(err) => err.to_compile_error(),
    })
}

fn impl_my_trait(ast: DeriveInput) -> Result<TokenStream2> {
    Ok({
        let name = ast.ident;
        let fields = match ast.data {
            Data::Enum(DataEnum {
                enum_token: token::Enum { span },
                ..
            })
            | Data::Union(DataUnion {
                union_token: token::Union { span },
                ..
            }) => {
                return Err(Error::new(span, "Expected a `struct`"));
            }

            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => it,

            Data::Struct(_) => {
                return Err(Error::new(
                    Span::call_site(),
                    "Expected a `struct` with named fields",
                ));
            }
        };

        let prost_fields: Vec<ProstField> = fields.named.into_iter().map(ProstField::new).collect();

        let data_expanded_members = prost_fields.iter().map(|field| {
            let field_name_str = LitStr::new(&field.name.to_string(), field.span);
            let datatype = &field.arrow_datatype();
            let nullable = &field.nullable;
            quote_spanned! { field.span=>
                             ::arrow_schema::Field::new(
                                 #field_name_str,
                                 #datatype,
                                 #nullable,
                             )
            }
        });

        let builder_struct_members = prost_fields.iter().map(|field| {
            let field_name = &field.name;
            let inner_type = &field.inner_type;
            let into_arrow_type = quote!(<#inner_type as ::prost_arrow::ToArrow>);
            let builder_type = if field.array {
                quote!(::prost_arrow::ArrowListBuilder::<#inner_type>)
            } else {
                quote!(#into_arrow_type::Builder)
            };
            quote_spanned! {
                field.span=> #field_name: #builder_type
            }
        });

        let builder_struct_initializers = prost_fields.iter().map(|field| {
            let field_name = &field.name;
            let inner_type = &field.inner_type;
            let into_arrow_type = quote!(<#inner_type as ::prost_arrow::ToArrow>);
            let builder_type = if field.array {
                quote!(::prost_arrow::ArrowListBuilder::<#inner_type>)
            } else {
                quote!(#into_arrow_type::Builder)
            };
            quote_spanned! {
                field.span=> #field_name: #builder_type::new_with_capacity(capacity)
            }
        });

        let builder_append_exprs = prost_fields.iter().map(|field| {
            let field_name = &field.name;

            if field.nullable {
                quote_spanned! {
                    field.span=> self.#field_name.append_option(value.#field_name)
                }
            } else {
                quote_spanned! {
                    field.span=> self.#field_name.append_value(value.#field_name)
                }
            }
        });

        let builder_append_none_exprs = prost_fields.iter().map(|field| {
            let field_name = &field.name;

            quote_spanned! {
                field.span=> self.#field_name.append_option(None)
            }
        });

        let fields_vec = quote! {
            ::arrow_schema::Fields::from(vec![
                #(#data_expanded_members ,)*
            ])
        };

        let finish_accessors = prost_fields.iter().map(|field| {
            let field_name = &field.name;

            quote_spanned! {
                field.span => self.#field_name.finish()
            }
        });

        let finish_cloned_accessors = prost_fields.iter().map(|field| {
            let field_name = &field.name;

            quote_spanned! {
                field.span => self.#field_name.finish_cloned()
            }
        });

        let builder_name = Ident::new(format!("{}Builder", name.to_string()).as_str(), name.span());

        quote! {
            pub struct #builder_name {
                null_buffer_builder: ::arrow_buffer::NullBufferBuilder,
                #(#builder_struct_members ,)*
            }

            impl ::prost_arrow::ToArrow for #name {
                type Item = #name;
                type Builder = #builder_name;

                fn to_datatype()
                  -> ::arrow_schema::DataType
                {
                    ::arrow_schema::DataType::Struct(#fields_vec)
                }
            }

            impl ::prost_arrow::ArrowBuilder<#name> for #builder_name {
                fn new_with_capacity(capacity: usize) -> Self {
                    Self{
                        null_buffer_builder: ::arrow_buffer::NullBufferBuilder::new(capacity),
                        #(#builder_struct_initializers ,)*
                    }
                }

                fn append_value(&mut self, value: #name) {
                    #(#builder_append_exprs ;)*
                    self.null_buffer_builder.append(true);
                }

                fn append_option(&mut self, value: Option<#name>) {
                    match value {
                        Some(v) => {
                            self.append_value(v);
                        },
                        None => {
                            #(#builder_append_none_exprs ;)*
                            self.null_buffer_builder.append(false);
                        },
                    }
                }
            }

            impl ::arrow_array::builder::ArrayBuilder for #builder_name {
                fn len(&self) -> usize {
                    self.null_buffer_builder.len()
                }

                fn finish(&mut self) -> ::arrow_array::ArrayRef {
                    let fields = #fields_vec;
                    let arrays = vec![
                        #(#finish_accessors ,)*
                    ];
                    let nulls = self.null_buffer_builder.finish();
                    ::std::sync::Arc::new(::arrow_array::StructArray::new(fields, arrays, nulls))
                }

                fn finish_cloned(&self) -> ::arrow_array::ArrayRef {
                    let fields = #fields_vec;
                    let arrays = vec![
                        #(#finish_cloned_accessors ,)*
                    ];
                    let nulls = self.null_buffer_builder.finish_cloned();
                    ::std::sync::Arc::new(::arrow_array::StructArray::new(fields, arrays, nulls))
                }

                fn as_any(&self) -> &dyn ::std::any::Any {
                    self
                }

                fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                    self
                }

                fn into_box_any(self: Box<Self>) -> Box<dyn ::std::any::Any> {
                    self
                }
            }
        }
    })
}

struct ProstField {
    span: Span,
    name: Ident,
    inner_type: TokenStream2,
    nullable: bool,
    array: bool,
}

impl ProstField {
    fn new(field: Field) -> Self {
        let (inner_type, nullable, array) = match &field.ty {
            Type::Path(path) => {
                let last = path.path.segments.last().expect("has last");

                // if Vec<u8> then inner should be Vec<u8> and array is false

                let inner = match &last.arguments {
                    PathArguments::AngleBracketed(args) => args
                        .args
                        .first()
                        .expect("has one type argument")
                        .into_token_stream(),
                    _ => path.into_token_stream(),
                };

                let last_ident = last.ident.to_string();
                let is_vec = last_ident.as_str() == "Vec";
                let is_binary = is_vec && inner.to_string() == "u8";
                let nullable = last_ident.as_str() == "Option";

                let (inner, array) = if is_binary {
                    (last.into_token_stream(), false)
                } else {
                    (inner, is_vec)
                };

                (inner, nullable, array)
            }

            other => (other.into_token_stream(), false, false),
        };

        Self {
            span: field.span(),
            name: field.ident.expect("field is named"),
            inner_type,
            nullable,
            array,
        }
    }

    fn arrow_datatype(&self) -> TokenStream2 {
        let inner = &self.inner_type;

        if self.array {
            quote_spanned! { self.span => ::arrow_schema::DataType::List(
                ::std::sync::Arc::new(::arrow_schema::Field::new_list_field(
                    <#inner as ::prost_arrow::ToArrow>::to_datatype(),
                    true,
                )))
            }
        } else {
            quote_spanned!(self.span => <#inner as ::prost_arrow::ToArrow>::to_datatype())
        }
    }
}
