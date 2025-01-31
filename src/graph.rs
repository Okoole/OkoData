use crate::{Result, Node, Relationship, Error};
use kuzu::{Database, SystemConfig, Connection, Value};
use std::path::Path;
use std::sync::Arc;
use serde_json::json;

/// Main interface for interacting with a KuzuDB graph database
#[derive(Debug)]
pub struct KuzuGraph {
    /// The underlying KuzuDB database instance
    db: Arc<Database>,
}

/// Represents an active database transaction
pub struct Transaction<'db> {
    connection: Connection<'db>,
    committed: bool,
}

impl<'db> Transaction<'db> {
    /// Creates a new transaction
    fn new(connection: Connection<'db>) -> Self {
        Self {
            connection,
            committed: false,
        }
    }

    /// Commits the transaction
    pub fn commit(&mut self) -> Result<()> {
        if !self.committed {
            self.connection.query("COMMIT")?;
            self.committed = true;
        }
        Ok(())
    }

    /// Rolls back the transaction
    pub fn rollback(&mut self) -> Result<()> {
        if !self.committed {
            self.connection.query("ROLLBACK")?;
            self.committed = true;
        }
        Ok(())
    }

    /// Gets the underlying connection
    pub fn connection(&self) -> &Connection<'db> {
        &self.connection
    }
}

impl<'db> Drop for Transaction<'db> {
    fn drop(&mut self) {
        if !self.committed {
            // Attempt to rollback if not committed
            let _ = self.rollback();
        }
    }
}

impl KuzuGraph {
    /// Creates a new KuzuGraph instance with the specified database path
    /// 
    /// # Arguments
    /// * `path` - Path to the KuzuDB database directory
    /// 
    /// # Returns
    /// * `Result<Self>` - A new KuzuGraph instance or an error
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Arc::new(Database::new(path, SystemConfig::default())?);
        Ok(Self { db })
    }

    /// Gets a new connection to the database
    /// 
    /// # Returns
    /// * `Result<Connection>` - A new database connection or an error
    pub fn get_connection(&self) -> Result<Connection> {
        Ok(Connection::new(&*self.db)?)
    }

    /// Create a node in the graph
    pub fn create_node<N: Node>(&self, node: &N) -> Result<()> {
        let label = N::label();
        let props = node.properties();
        let conn = self.get_connection()?;
        
        // First create the node table if it doesn't exist
        let mut columns = Vec::new();
        if let serde_json::Value::Object(obj) = &props {
            for (key, value) in obj {
                let col_type = match value {
                    serde_json::Value::String(_) => "STRING",
                    serde_json::Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            "INT64"
                        } else {
                            "DOUBLE"
                        }
                    },
                    serde_json::Value::Bool(_) => "BOOL",
                    _ => continue, // Skip unsupported types
                };
                columns.push(format!("{} {}", key, col_type));
            }
        }
        
        let create_table = format!(
            "CREATE NODE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY({}))",
            label,
            columns.join(", "),
            N::primary_key_field()
        );
        
        // Execute table creation
        conn.query(&create_table)?;
        
        // Format properties as individual SET clauses
        let mut props_list = Vec::new();
        if let serde_json::Value::Object(obj) = props {
            for (key, value) in obj {
                let value_str = match value {
                    serde_json::Value::String(s) => format!("'{}'", s),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => continue, // Skip unsupported types
                };
                props_list.push(format!("{}: {}", key, value_str));
            }
        }
        
        // First try to delete any existing node with the same ID
        let delete_query = format!(
            "MATCH (n:{}) WHERE n.{} = '{}' DELETE n",
            label,
            N::primary_key_field(),
            node.primary_key()
        );
        
        conn.query(&delete_query)?;
        
        // Then create the new node
        let query = format!(
            "CREATE (:{} {{{}}})",
            label,
            props_list.join(", ")
        );
        
        conn.query(&query)?;
        Ok(())
    }

    /// Find a node by its primary key
    pub fn find_node<N: Node>(&self, id: &str) -> Result<Option<N>> {
        let label = N::label();
        let pk_field = N::primary_key_field();
        let conn = self.get_connection()?;
        
        let query = format!(
            "MATCH (n:{}) WHERE n.{} = '{}' RETURN n",
            label, pk_field, id
        );
        
        let result = conn.query(&query)?;
        let mut rows = result.into_iter();
        
        if let Some(row) = rows.next() {
            if let Value::Node(node) = &row[0] {
                // Convert node properties to JSON value
                let mut props = json!({});
                for (key, value) in node.get_properties() {
                    props[key] = match value {
                        Value::String(s) => json!(s),
                        Value::Int64(n) => json!(n),
                        Value::UInt64(n) => json!(n),
                        Value::Double(n) => json!(n),
                        Value::Bool(b) => json!(b),
                        _ => continue, // Skip unsupported types
                    };
                }
                
                // Deserialize from JSON value
                Ok(Some(serde_json::from_value(props)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Create a relationship between nodes
    pub fn create_relationship<R: Relationship>(&self, rel: &R) -> Result<()> {
        let label = R::label();
        let props = rel.properties();
        let conn = self.get_connection()?;
        
        // Format properties as individual SET clauses
        let mut props_list = Vec::new();
        if let serde_json::Value::Object(obj) = props {
            for (key, value) in obj {
                // Skip the from/to fields
                if key == "from" || key == "to" {
                    continue;
                }
                let value_str = match value {
                    serde_json::Value::String(s) => format!("'{}'", s),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => continue, // Skip unsupported types
                };
                props_list.push(format!("{}: {}", key, value_str));
            }
        }
        
        let query = format!(
            "MATCH (a:User), (b:User) 
             WHERE a.id = '{}' AND b.id = '{}'
             CREATE (a)-[r:{} {{{}}}]->(b)",
            rel.from(),
            rel.to(),
            label,
            props_list.join(", ")
        );
        
        conn.query(&query)?;
        Ok(())
    }

    /// Execute a raw Cypher query
    pub fn query(&self, query: &str) -> Result<kuzu::QueryResult> {
        let conn = self.get_connection()?;
        Ok(conn.query(query)?)
    }

    pub fn create_relationship_table<R: Relationship>(&self) -> Result<()> {
        let label = R::label();
        let from_label = R::from_node_label();
        let to_label = R::to_node_label();
        
        // Only include non-endpoint properties
        let query = format!(
            "CREATE REL TABLE IF NOT EXISTS {} (FROM {} TO {}, since INT64)",
            label,
            from_label,
            to_label
        );
        
        let conn = self.get_connection()?;
        conn.query(&query)?;
        Ok(())
    }

    /// Create a node table if it doesn't exist
    pub fn create_node_table<N: Node>(&self) -> Result<()> {
        let label = N::label();
        let pk_field = N::primary_key_field();
        let conn = self.get_connection()?;
        
        // Get property types from the Node trait
        let mut columns = Vec::new();
        if let serde_json::Value::Object(obj) = N::property_types() {
            for (key, value) in obj {
                let col_type = match value {
                    serde_json::Value::String(_) => "STRING",
                    serde_json::Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            "INT64"
                        } else {
                            "DOUBLE"
                        }
                    },
                    serde_json::Value::Bool(_) => "BOOL",
                    _ => continue, // Skip unsupported types
                };
                columns.push(format!("{} {}", key, col_type));
            }
        }
        
        let query = format!(
            "CREATE NODE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY({}))",
            label,
            columns.join(", "),
            pk_field
        );
        
        conn.query(&query)?;
        Ok(())
    }

    /// Begins a new transaction
    pub fn begin_transaction(&self) -> Result<Transaction> {
        let conn = self.get_connection()?;
        conn.query("BEGIN TRANSACTION")?;
        Ok(Transaction::new(conn))
    }

    /// Execute a function within a transaction
    /// The transaction will be committed if the function returns Ok,
    /// and rolled back if it returns Err
    pub fn transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Transaction) -> Result<T>,
    {
        let mut txn = self.begin_transaction()?;
        match f(&txn) {
            Ok(result) => {
                txn.commit()?;
                Ok(result)
            }
            Err(e) => {
                txn.rollback()?;
                Err(e)
            }
        }
    }

    /// Create a node within a transaction
    pub fn create_node_in_transaction<N: Node>(&self, txn: &Transaction, node: &N) -> Result<()> {
        let label = N::label();
        let props = node.properties();
        
        // Convert properties to a Cypher-compatible format
        let props_str = if let serde_json::Value::Object(obj) = &props {
            let props: Vec<String> = obj.iter()
                .map(|(k, v)| {
                    let value_str = match v {
                        serde_json::Value::String(s) => format!("'{}'", s.replace("'", "\\'")),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => return Err(Error::InvalidPropertyType("Unsupported property type".into())),
                    };
                    Ok(format!("{}: {}", k, value_str))
                })
                .collect::<Result<Vec<_>>>()?;
            props.join(", ")
        } else {
            return Err(Error::InvalidPropertyType("Properties must be an object".into()));
        };

        let query = format!("CREATE (n:{} {{{}}}) RETURN n", label, props_str);
        txn.connection().query(&query)?;
        Ok(())
    }
}
