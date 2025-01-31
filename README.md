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

The project documentation is currently under development. In the meantime, you can:

- Check the inline code documentation using `cargo doc --open`
- View example usage in the `tests/` directory
- Visit our [GitHub repository](https://github.com/Okoole/OkoData) for the latest updates

Documentation roadmap:
- [ ] API Documentation
- [ ] User Guide
- [ ] Examples Collection
- [ ] Integration Guides
- [ ] Best Practices

Stay tuned for updates!

## Contributing

We welcome contributions! Here's how you can help:

1. 📋 **Check the Roadmap**: Review our [ROADMAP.md](ROADMAP.md) file to see:
   - Features that need implementation
   - What's currently in progress
   - Planned enhancements
   - Release planning

2. 🌿 **Branching Guidelines**:
   - Create a feature branch for each contribution:
     ```bash
     git checkout -b feature/feature-name
     # or
     git checkout -b enhancement/enhancement-name
     git checkout -b bugfix/bug-name
     ```
   - Use prefixes to categorize your branches:
     - `feature/` for new features
     - `enhancement/` for improvements
     - `bugfix/` for bug fixes
     - `docs/` for documentation changes

3. 🔄 **Development Workflow**:
   - Make your changes in your feature branch
   - Write or update tests as needed
   - Follow the existing code style
   - Commit your changes with clear messages
   - Push your branch and create a Pull Request

4. 📝 **Before Submitting**:
   - Ensure all tests pass
   - Update documentation if needed
   - Add your feature to ROADMAP.md if completed
   - Describe your changes in the PR

For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of [KuzuDB](https://kuzudb.com)
- Inspired by modern ORM/OGM patterns 