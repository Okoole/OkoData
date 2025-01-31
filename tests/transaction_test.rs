#[cfg(test)]
mod transaction_tests {
    use kuzu_ogm::{Node, KuzuGraph, Error, Result};
    use serde::{Serialize, Deserialize};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[derive(Node, Debug, PartialEq, Serialize, Deserialize)]
    #[label("Account")]
    struct Account {
        #[id]
        id: String,
        #[property]
        balance: i64,
    }

    fn setup_test_db() -> KuzuGraph {
        let test_num = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let test_db_path = PathBuf::from(format!("test_transaction_db_{}", test_num));
        
        if test_db_path.exists() {
            fs::remove_dir_all(&test_db_path).unwrap();
        }
        fs::create_dir_all(&test_db_path).unwrap();
        
        let db = KuzuGraph::new(test_db_path.to_str().unwrap()).unwrap();
        
        // Create the Account table using OGM
        db.create_node_table::<Account>().unwrap();
        
        // Initialize the database by creating a dummy node and then deleting it
        let dummy = Account {
            id: "dummy".to_string(),
            balance: 0,
        };
        db.create_node(&dummy).unwrap();
        
        db
    }

    #[test]
    fn test_transaction_commit() {
        let db = setup_test_db();
        
        // Create two accounts in a transaction
        let mut txn = db.begin_transaction().unwrap();
        
        let account1 = Account {
            id: "acc1".to_string(),
            balance: 1000,
        };
        
        let account2 = Account {
            id: "acc2".to_string(),
            balance: 2000,
        };
        
        db.create_node_in_transaction(&txn, &account1).unwrap();
        db.create_node_in_transaction(&txn, &account2).unwrap();
        
        txn.commit().unwrap();
        
        // Verify both accounts exist
        let acc1: Option<Account> = db.find_node("acc1").unwrap();
        let acc2: Option<Account> = db.find_node("acc2").unwrap();
        
        assert!(acc1.is_some());
        assert!(acc2.is_some());
        assert_eq!(acc1.unwrap().balance, 1000);
        assert_eq!(acc2.unwrap().balance, 2000);
    }

    #[test]
    fn test_transaction_rollback() {
        let db = setup_test_db();
        
        // Start a transaction
        let mut txn = db.begin_transaction().unwrap();
        
        let account = Account {
            id: "acc3".to_string(),
            balance: 3000,
        };
        
        db.create_node_in_transaction(&txn, &account).unwrap();
        
        // Rollback the transaction
        txn.rollback().unwrap();
        
        // Verify account doesn't exist
        let acc: Option<Account> = db.find_node("acc3").unwrap();
        assert!(acc.is_none());
    }

    #[test]
    fn test_transaction_scope() {
        let db = setup_test_db();
        
        // Test transaction scope using closure
        let result: Result<()> = db.transaction(|txn| {
            let account = Account {
                id: "acc4".to_string(),
                balance: 4000,
            };
            
            db.create_node_in_transaction(txn, &account)?;
            
            Ok(())
        });
        
        assert!(result.is_ok());
        
        // Verify account exists
        let acc: Option<Account> = db.find_node("acc4").unwrap();
        assert!(acc.is_some());
        assert_eq!(acc.unwrap().balance, 4000);
    }

    #[test]
    fn test_transaction_error_handling() {
        let db = setup_test_db();
        
        // Test transaction with error
        let result: Result<()> = db.transaction(|txn| {
            let account = Account {
                id: "acc5".to_string(),
                balance: 5000,
            };
            
            db.create_node_in_transaction(txn, &account)?;
            
            // Simulate an error
            Err(Error::InvalidPropertyType("Simulated error".into()))
        });
        
        assert!(result.is_err());
        
        // Verify account doesn't exist due to rollback
        let acc: Option<Account> = db.find_node("acc5").unwrap();
        assert!(acc.is_none());
    }

    #[test]
    fn test_transaction_automatic_rollback() {
        let db = setup_test_db();
        
        // Create a transaction scope that will be dropped without commit
        {
            let txn = db.begin_transaction().unwrap();
            
            let account = Account {
                id: "acc6".to_string(),
                balance: 6000,
            };
            
            db.create_node_in_transaction(&txn, &account).unwrap();
            
            // Transaction will be dropped here without commit
        }
        
        // Verify account doesn't exist due to automatic rollback
        let acc: Option<Account> = db.find_node("acc6").unwrap();
        assert!(acc.is_none());
    }
} 