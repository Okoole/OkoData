use serde::{Serialize, Deserialize};

/// Trait for node types in the graph
pub trait Node: Serialize + for<'de> Deserialize<'de> {
    /// Get the label for this node type
    fn label() -> &'static str;
    
    /// Get the primary key field name
    fn primary_key_field() -> &'static str;
    
    /// Get the primary key value
    fn primary_key(&self) -> String;
    
    /// Get the properties as a JSON value
    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    /// Get the property types for this node type
    fn property_types() -> serde_json::Value;
}

/// Trait for relationship types in the graph
pub trait Relationship: Serialize + for<'de> Deserialize<'de> {
    /// Get the label for this relationship type
    fn label() -> &'static str;
    
    /// Get the source node ID
    fn from(&self) -> String;
    
    /// Get the target node ID 
    fn to(&self) -> String;
    
    /// Get the properties as a JSON value
    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    /// Get the source node label
    fn from_node_label() -> &'static str;
    
    /// Get the target node label
    fn to_node_label() -> &'static str;
    
    /// Get the property definitions
    fn property_definitions() -> Vec<String>;
} 