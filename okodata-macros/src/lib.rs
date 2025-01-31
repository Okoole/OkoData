extern crate proc_macro;

mod node;
mod relationship;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Derive macro for Node types
#[proc_macro_derive(Node, attributes(label, id, property))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    node::derive_node(input)
}

/// Derive macro for Relationship types
#[proc_macro_derive(Relationship, attributes(label, from_node, to_node, property))]
pub fn derive_relationship(input: TokenStream) -> TokenStream {
    relationship::derive_relationship(input)
}

/// Attribute macro for marking a field as a from_node
#[proc_macro_attribute]
pub fn from_node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro for marking a field as a to_node
#[proc_macro_attribute]
pub fn to_node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro for marking a field as a property
#[proc_macro_attribute]
pub fn property(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro for marking a field as an id
#[proc_macro_attribute]
pub fn id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro for specifying a label
#[proc_macro_attribute]
pub fn label(attr: TokenStream, item: TokenStream) -> TokenStream {
    let label = parse_macro_input!(attr as LitStr);
    let item = parse_macro_input!(item as syn::Item);
    
    let expanded = quote! {
        #[doc = #label]
        #item
    };
    
    TokenStream::from(expanded)
} 