use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, FieldsNamed, Result};

use crate::parse::find_attr;

use super::parse::{FieldType, StructField, TypeAttribute};

/// Implementation of CommandModel derive macro
pub fn impl_command_model(input: DeriveInput, fields: FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let fields = StructField::from_fields(fields)?;

    let partial = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?.partial,
        None => false,
    };

    let field_unknown = field_unknown(partial);
    let fields_init = fields.iter().map(field_init);
    let fields_match_arms = fields.iter().map(field_match_arm);
    let fields_constructor = fields.iter().map(field_constructor);

    Ok(quote! {
        impl ::twilight_interactions::command::CommandModel for #ident {
            fn from_interaction(
                data: ::twilight_interactions::command::CommandInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                if data.options.is_empty() {
                    return std::result::Result::Err(::twilight_interactions::error::ParseError::EmptyOptions);
                }

                #(#fields_init)*

                for opt in data.options {
                    match &*opt.name {
                        #(#fields_match_arms,)*
                        other => #field_unknown
                    }
                }

                ::std::result::Result::Ok(Self { #(#fields_constructor),* })
            }
        }
    })
}

/// Generate field initialization variables
fn field_init(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    quote!(let mut #ident = None;)
}

/// Generate field match arm
fn field_match_arm(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let name = field.attributes.name_default(ident.to_string());
    let span = field.span;

    quote_spanned! {span=>
        #name => match ::twilight_interactions::command::CommandOption::from_option(opt.value, data.resolved.as_deref()) {
            ::std::result::Result::Ok(value) => #ident = Some(value),
            ::std::result::Result::Err(kind) => {
                return ::std::result::Result::Err(
                    ::twilight_interactions::error::ParseError::Option(
                        ::twilight_interactions::error::ParseOptionError {
                            field: ::std::convert::From::from(#name),
                            kind,
                    })
                )
            }
        }
    }
}

/// Generate field constructor
fn field_constructor(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let ident_str = ident.to_string();

    match field.kind {
        FieldType::Required => quote! {
            #ident: match #ident {
                Some(value) => value,
                None => return Err(::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(#ident_str),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::RequiredField
                }))
            }
        },
        FieldType::Optional => quote!(#ident),
    }
}

/// Generate unknown field match arm
fn field_unknown(partial: bool) -> TokenStream {
    if partial {
        quote!(continue)
    } else {
        quote! {
            return ::std::result::Result::Err(
                ::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(other),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::UnknownField,
                })
            )
        }
    }
}