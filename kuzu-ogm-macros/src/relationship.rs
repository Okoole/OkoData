use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

pub fn derive_relationship(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // Get the struct fields
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Relationship derive only works on structs with named fields"),
        },
        _ => panic!("Relationship derive only works on structs"),
    };

    // Find the from/to fields and property fields
    let mut from_field = None;
    let mut to_field = None;
    let mut property_fields = Vec::new();
    
    for field in fields.iter() {
        for attr in &field.attrs {
            let path = attr.path();
            let ident = path.get_ident().map(|i| i.to_string());
            match ident.as_deref() {
                Some("from_node") => {
                    from_field = Some(field.ident.clone().unwrap());
                },
                Some("to_node") => {
                    to_field = Some(field.ident.clone().unwrap());
                },
                Some("property") => {
                    property_fields.push((field.ident.clone().unwrap(), &field.ty));
                },
                _ => {}
            }
        }
    }

    let from_field = from_field.expect("Missing required attribute #[from_node] on relationship field");
    let to_field = to_field.expect("Missing required attribute #[to_node] on relationship field");

    // Get the label from attributes or use struct name
    let label = input.attrs.iter()
        .find(|attr| attr.path().is_ident("label"))
        .map(|attr| attr.parse_args::<syn::LitStr>().unwrap().value())
        .unwrap_or_else(|| name.to_string());

    // Generate property definitions
    let property_defs = property_fields.iter().map(|(name, ty)| {
        let type_str = match quote!(#ty).to_string().as_str() {
            "String" => "STRING",
            "u32" | "i32" | "u64" | "i64" => "INT64",
            "f32" | "f64" => "DOUBLE",
            "bool" => "BOOL",
            _ => panic!("Unsupported property type: {}", quote!(#ty)),
        };
        format!("{} {}", name, type_str)
    }).collect::<Vec<_>>();

    let expanded = quote! {
        impl Relationship for #name {
            fn label() -> &'static str {
                #label
            }

            fn from(&self) -> String {
                self.#from_field.to_string()
            }

            fn to(&self) -> String {
                self.#to_field.to_string()
            }

            fn from_node_label() -> &'static str {
                "User"
            }

            fn to_node_label() -> &'static str {
                "User"
            }

            fn property_definitions() -> Vec<String> {
                vec![#(#property_defs.to_string()),*]
            }
        }
    };

    TokenStream::from(expanded)
} 