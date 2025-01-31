# OkoData

A powerful, type-safe Object-Graph Mapper (OGM) for KuzuDB written in Rust.


> ⚠️ **Note**: This crate is under active development. While we're working hard to make it production-ready, the API may undergo changes. We encourage you to try it out and provide feedback!


## Overview

OkoData is a high-performance Object-Graph Mapper (OGM) that provides a seamless interface between Rust applications and KuzuDB. It enables developers to work with graph data using native Rust types while maintaining the full power and flexibility of graph queries.

## Features

- 🚀 **Type-safe Graph Operations**: Leverage Rust's type system for safe graph operations
- 📝 **Declarative Model Definitions**: Define nodes and relationships using simple Rust structs and attributes
- 🔄 **Full CRUD Support**: Complete Create, Read, Update, Delete operations for both nodes and relationships
- 🔍 **Fluent Query Builder**: Intuitive and type-safe query building interface
- 🌐 **Advanced Graph Traversals**: Support for complex graph traversals and pathfinding
- 🔒 **Async Transaction Support**: Full async/await support with transaction management
- 📦 **Batch Processing**: Optimized bulk operations for better performance
- 🔄 **Real-time Updates**: Subscribe to graph changes with async streams
- ⚡ **High Performance**: Optimized query execution and connection pooling
- 🛠️ **Custom Extensions**: Execute raw Cypher queries when needed

## Quick Start

Add OkoData to your `Cargo.toml`:

```toml
[dependencies]
okodata = "0.1.0"
```

### Basic Usage

```rust
use okodata::prelude::*;

// Define a node
#[derive(Node)]
#[label = "User"]
struct User {
    #[id]
    id: String,
    
    #[property]
    name: String,
    
    #[property]
    age: u32,
}

// Define a relationship
#[derive(Relationship)]
#[label = "FOLLOWS"]
struct Follows {
    #[start]
    from: String,
    
    #[end]
    to: String,
    
    #[property]
    since: u32,
}

// Connect and perform operations
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = KuzuGraph::new("database.db").await?;
    
    // Create a user
    let user = User {
        id: "u1".to_string(),
        name: "Alice".to_string(),
        age: 25,
    };
    
    db.insert(&user).await?;
    
    // Query users
    let users = db.query::<User>()
        .filter(|user| user.age.gt(20))
        .limit(10)
        .fetch()
        .await?;
        
    Ok(())
}
```

## Advanced Features

### Transaction Management

```rust
let txn = db.begin_transaction().await?;

// Perform multiple operations
let user = User { id: "u3".to_string(), name: "Bob".to_string(), age: 30 };
txn.insert(&user).await?;

// Commit or rollback
txn.commit().await?;
```

### Real-time Updates

```rust
let stream = db.watch::<User>()
    .filter(|user| user.age.gt(20))
    .subscribe();

while let Some(updated_users) = stream.next().await {
    println!("{:?}", updated_users);
}
```

## Documentation

For detailed documentation and examples, visit:
- [API Documentation](https://docs.rs/okodata)
- [User Guide](https://github.com/yourusername/okodata/wiki)
- [Examples](https://github.com/yourusername/okodata/tree/main/examples)

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of [KuzuDB](https://kuzudb.com)
- Inspired by modern ORM/OGM patterns 