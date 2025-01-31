#[cfg(test)]
use crate::Node;
#[cfg(test)]
use std::marker::PhantomData;

/// A builder for constructing type-safe Cypher queries
#[derive(Default, Debug, Clone)]
#[cfg(test)]
pub struct QueryBuilder<N> {
    /// Phantom data to hold the node type
    node_type: PhantomData<N>,
    /// List of WHERE conditions
    conditions: Vec<String>,
    /// Optional LIMIT clause value
    limit: Option<usize>,
    /// Optional SKIP clause value
    skip: Option<usize>,
}

#[cfg(test)]
impl<N: Node> QueryBuilder<N> {
    /// Creates a new query builder for the given node type
    /// 
    /// # Returns
    /// * A new QueryBuilder instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            node_type: PhantomData,
            conditions: Vec::new(),
            limit: None,
            skip: None,
        }
    }

    /// Adds a WHERE condition to the query
    /// 
    /// # Arguments
    /// * `condition` - The condition to add to the WHERE clause
    /// 
    /// # Returns
    /// * Self for method chaining
    #[must_use]
    pub fn where_clause<S: Into<String>>(mut self, condition: S) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Limits the number of results
    /// 
    /// # Arguments
    /// * `limit` - Maximum number of results to return
    /// 
    /// # Returns
    /// * Self for method chaining
    #[must_use]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Skips a number of results
    /// 
    /// # Arguments
    /// * `skip` - Number of results to skip
    /// 
    /// # Returns
    /// * Self for method chaining
    #[must_use]
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Builds the Cypher query string
    /// 
    /// # Returns
    /// * A string containing the complete Cypher query
    pub fn build(&self) -> String {
        let mut query = format!("MATCH (n:{})", N::label());
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        query.push_str(" RETURN n");
        
        if let Some(skip) = self.skip {
            query.push_str(&format!(" SKIP {}", skip));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct TestNode {
        id: String,
        name: String,
    }

    impl Node for TestNode {
        fn label() -> &'static str {
            "TestNode"
        }

        fn primary_key_field() -> &'static str {
            "id"
        }

        fn primary_key(&self) -> String {
            self.id.clone()
        }

        fn property_types() -> serde_json::Value {
            json!({
                "id": "",
                "name": ""
            })
        }
    }

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::<TestNode>::new()
            .where_clause("n.age > 18")
            .limit(10)
            .skip(5)
            .build();

        assert_eq!(
            query,
            "MATCH (n:TestNode) WHERE n.age > 18 RETURN n SKIP 5 LIMIT 10"
        );
    }
}
