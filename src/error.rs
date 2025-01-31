use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] kuzu::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Node not found with id: {0}")]
    NodeNotFound(String),

    #[error("Invalid property type: {0}")]
    InvalidPropertyType(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Query error: {0}")]
    Query(String),
} 