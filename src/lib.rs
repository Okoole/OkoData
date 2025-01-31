//! OkoData - A Graph Database ORM
//! 
//! This crate provides a high-level, type-safe interface for working with KuzuDB graphs in Rust.

mod error;
mod graph;
mod query;
mod traits;

pub use error::Error;
pub use graph::KuzuGraph;
pub use traits::{Node, Relationship};

// Re-export derive macros and attributes
pub use okodata_macros::{Node, Relationship};
pub use okodata_macros::{id, property, label, from_node, to_node};

// Re-export important types from kuzu
pub use kuzu::{Database, SystemConfig, Connection, Value};

/// Result type for OkoData operations
pub type Result<T> = std::result::Result<T, Error>; 