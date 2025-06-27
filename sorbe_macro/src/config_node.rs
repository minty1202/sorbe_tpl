use super::ConfigField;
use indexmap::IndexMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Token, Type, punctuated::Punctuated};

pub type ConfigTree = IndexMap<String, ConfigNode>;

#[derive(Debug, Clone)]
pub enum ConfigNode {
    Leaf(Leaf),
    Branch(Branch),
}

#[derive(Debug, Clone)]
pub struct Leaf {
    ty: Type,
}

#[derive(Debug, Clone)]
pub struct Branch {
    children: ConfigTree,
}

impl ConfigNode {
    pub fn to_field_token(&self, field_name: &str) -> TokenStream {
        match self {
            ConfigNode::Leaf(leaf) => leaf.to_field_token(field_name),
            ConfigNode::Branch(branch) => branch.to_field_token(field_name),
        }
    }

    pub fn build_tree(fields: &Punctuated<ConfigField, Token![,]>) -> ConfigTree {
        let mut tree = IndexMap::new();

        for field in fields {
            let segments: Vec<String> = field.path.segments.iter().map(|s| s.to_string()).collect();
            Self::insert_into_tree(&mut tree, &segments, field.ty.clone());
        }

        tree
    }

    fn insert_into_tree(tree: &mut ConfigTree, path: &[String], ty: Type) {
        if path.len() == 1 {
            let leaf = Leaf { ty };
            tree.insert(path[0].clone(), ConfigNode::Leaf(leaf));
        } else {
            let entry = tree.entry(path[0].clone()).or_insert_with(|| {
                ConfigNode::Branch(Branch {
                    children: IndexMap::new(),
                })
            });

            if let ConfigNode::Branch(branch) = entry {
                Self::insert_into_tree(&mut branch.children, &path[1..], ty);
            } else {
                panic!("Expected a Branch node, found {:?}", entry);
            }
        }
    }
}

impl Leaf {
    pub fn to_field_token(&self, field_name: &str) -> TokenStream {
        let name = syn::Ident::new(field_name, proc_macro2::Span::call_site());
        let ty = &self.ty;
        quote! { #name: #ty }
    }
}

impl Branch {
    pub fn to_field_token(&self, field_name: &str) -> TokenStream {
        let name = syn::Ident::new(field_name, proc_macro2::Span::call_site());
        let type_name = syn::Ident::new(&capitalize(field_name), proc_macro2::Span::call_site());
        quote! { #name: #type_name }
    }

    pub fn generate_struct(&self, struct_name: &str) -> TokenStream {
        let fields: Vec<_> = self
            .children
            .iter()
            .map(|(name, node)| node.to_field_token(name))
            .collect();

        let struct_ident =
            syn::Ident::new(&capitalize(struct_name), proc_macro2::Span::call_site());
        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            struct #struct_ident {
                #(#fields,)*
            }
        }
    }

    pub fn collect_all_structs(&self, struct_name: &str) -> Vec<TokenStream> {
        let mut structs = vec![self.generate_struct(struct_name)];

        for (child_name, child_node) in &self.children {
            if let ConfigNode::Branch(branch) = child_node {
                structs.extend(branch.collect_all_structs(child_name));
            }
        }

        structs
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
