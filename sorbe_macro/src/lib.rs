mod config_node;
mod config_validator;

use config_node::ConfigNode;
use config_validator::{ConfigValidationError, ConfigValidator};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, Type, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct ConfigInput {
    struct_name: Ident,
    _arrow: Token![=>],
    _brace_token: syn::token::Brace,
    fields: Punctuated<ConfigField, Token![,]>,
}

impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ConfigInput {
            struct_name: input.parse()?,
            _arrow: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(ConfigField::parse, Token![,])?,
        })
    }
}

struct ConfigField {
    path: ConfigPath,
    _colon: Token![:],
    ty: Type,
}

impl Parse for ConfigField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ConfigField {
            path: input.parse()?,
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

struct ConfigPath {
    segments: Punctuated<Ident, Token![.]>,
}

impl Parse for ConfigPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut segments = Punctuated::new();
        segments.push(input.parse::<Ident>()?);

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            segments.push(input.parse::<Ident>()?);
        }

        Ok(ConfigPath { segments })
    }
}

#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as ConfigInput);

    let struct_name = &config.struct_name;
    let fields = &config.fields;

    if let Err(e) = ConfigValidator::validate(fields) {
        let error_message = match e {
            ConfigValidationError::Duplicate { key } => {
                format!("Duplicate key found: '{}'", key)
            }
            ConfigValidationError::KeyPathConflict { key } => {
                format!(
                    "Key path conflict: '{}' cannot have both direct value and nested fields",
                    key
                )
            }
        };

        return syn::Error::new(proc_macro2::Span::call_site(), error_message)
            .to_compile_error()
            .into();
    }

    let tree = ConfigNode::build_tree(fields);

    let mut all_nested_structs = Vec::new();
    for (name, node) in &tree {
        if let ConfigNode::Branch(branch) = node {
            all_nested_structs.extend(branch.collect_all_structs(name));
        }
    }

    let main_fields: Vec<_> = tree
        .iter()
        .map(|(name, node)| node.to_field_token(name))
        .collect();

    let output = quote! {

        #(#all_nested_structs)*

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        struct #struct_name {
            #(#main_fields,)*
        }
    };

    output.into()
}
