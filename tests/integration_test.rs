#[cfg(test)]
mod tests {
    use okodata::{Node, Relationship, KuzuGraph, Value};
    use serde::{Serialize, Deserialize};
    use std::fs;
    use tokio;
    use std::path::PathBuf;

    #[derive(Node, Debug, PartialEq, Serialize, Deserialize)]
    #[label("User")]
    struct User {
        #[id]
        id: String,
        #[property]
        name: String,
        #[property]
        age: u32,
    }

    #[derive(Relationship, Debug, Serialize, Deserialize)]
    #[label("FOLLOWS")]
    struct Follows {
        #[from_node]
        from: String,
        #[to_node]
        to: String,
        #[property]
        since: u32,
    }

    fn setup_test_db() -> KuzuGraph {
        let test_db_path = PathBuf::from("test_db");
        
        if test_db_path.exists() {
            fs::remove_dir_all(&test_db_path).unwrap();
        }
        fs::create_dir_all(&test_db_path).unwrap();
        
        println!("Creating test database at: {:?}", test_db_path);
        let db = KuzuGraph::new(test_db_path.to_str().unwrap()).unwrap();
        
        // Create tables first
        db.create_node_table::<User>().unwrap();
        db.create_relationship_table::<Follows>().unwrap();
        
        db
    }

    #[tokio::test]
    async fn test_crud_operations() {
        let db = setup_test_db();
        
        // Test Create
        let user = User {
            id: "u1".to_string(),
            name: "Alice".to_string(),
            age: 25,
        };
        
        db.create_node(&user).unwrap();
        
        // Test Read
        let fetched_user: Option<User> = db.find_node("u1").unwrap();
        assert!(fetched_user.is_some());
        assert_eq!(fetched_user.unwrap(), user);
        
        // Test Update
        let mut updated_user = user;
        updated_user.age = 26;
        
        // Note: Update operation needs to be implemented
        // For now we'll just create a new node which will overwrite the old one
        db.create_node(&updated_user).unwrap();
        
        let fetched_updated: Option<User> = db.find_node("u1").unwrap();
        assert_eq!(fetched_updated.unwrap().age, 26);
        
        // Note: Delete operation needs to be implemented
        // For now we'll just verify the node exists
        let exists: Option<User> = db.find_node("u1").unwrap();
        assert!(exists.is_some());
    }

    #[tokio::test]
    async fn test_relationships() {
        let db = setup_test_db();
        
        // Create tables first
        db.create_node_table::<User>().unwrap();
        db.create_relationship_table::<Follows>().unwrap();
        
        // Create users
        let alice = User {
            id: "u1".to_string(),
            name: "Alice".to_string(),
            age: 25,
        };
        
        let bob = User {
            id: "u2".to_string(),
            name: "Bob".to_string(),
            age: 30,
        };
        
        db.create_node(&alice).unwrap();
        db.create_node(&bob).unwrap();
        
        // Create relationship
        let follows = Follows {
            from: alice.id.clone(),
            to: bob.id.clone(),
            since: 2024,
        };
        
        db.create_relationship(&follows).unwrap();
        
        // Test relationship query
        let query = format!(
            "MATCH (u1:User)-[f:FOLLOWS]->(u2:User) 
             WHERE u2.id = '{}' 
             RETURN COUNT(*) as count",
            bob.id
        );
        
        let result = db.query(&query).unwrap();
        let mut rows = result.into_iter();
        let row = rows.next().unwrap();
        let count = match &row[0] {
            Value::Int64(n) => *n,
            _ => panic!("Expected Int64 value"),
        };
        assert_eq!(count, 1);
    }
} 
