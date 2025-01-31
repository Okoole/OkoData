use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // Get the struct fields
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Node derive only works on structs with named fields"),
        },
        _ => panic!("Node derive only works on structs"),
    };

    // Find the primary key field and collect property fields
    let mut primary_key_field = None;
    let mut property_fields = Vec::new();
    
    for field in fields.iter() {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;
        
        for attr in &field.attrs {
            if attr.path().is_ident("id") {
                primary_key_field = Some(field_name.clone());
            } else if attr.path().is_ident("property") {
                property_fields.push((field_name.clone(), field_type.clone()));
            }
        }
    }

    let primary_key_field = primary_key_field.expect("Node must have a field marked with #[id]");

    // Get the label from attributes or use struct name
    let label = input.attrs.iter()
        .find(|attr| attr.path().is_ident("label"))
        .map(|attr| attr.parse_args::<syn::LitStr>().unwrap().value())
        .unwrap_or_else(|| name.to_string());

    // Generate property type initialization code
    let property_type_inits = property_fields.iter().map(|(name, ty)| {
        let name_str = name.to_string();
        quote! {
            props.insert(#name_str.to_string(), match stringify!(#ty) {
                "String" | "&str" => serde_json::Value::String(String::new()),
                "i64" | "i32" | "i16" | "i8" | "u64" | "u32" | "u16" | "u8" => serde_json::Value::Number(0.into()),
                "bool" => serde_json::Value::Bool(false),
                _ => serde_json::Value::Null,
            });
        }
    });

    let expanded = quote! {
        impl Node for #name {
            fn label() -> &'static str {
                #label
            }

            fn primary_key_field() -> &'static str {
                stringify!(#primary_key_field)
            }

            fn primary_key(&self) -> String {
                self.#primary_key_field.to_string()
            }

            fn property_types() -> serde_json::Value {
                let mut props = std::collections::HashMap::new();
                props.insert(stringify!(#primary_key_field).to_string(), serde_json::Value::String(String::new()));
                #(#property_type_inits)*
                serde_json::Value::Object(serde_json::Map::from_iter(props))
            }
        }
    };

    TokenStream::from(expanded)
} 