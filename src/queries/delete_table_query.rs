use std::collections::HashMap;

use super::{Query, QuerySuccess};
use crate::{errors::DbErr, tokenizer::Token};

pub(crate) struct DeleteTableQuery {
    data: HashMap<String, Token>,
}

impl Query for DeleteTableQuery {
    fn new(data: HashMap<String, Token>) -> DeleteTableQuery {
        DeleteTableQuery { data }
    }

    fn execute(&self, database: &mut crate::database::Database) -> Result<QuerySuccess, DbErr> {
        if let Some(Token::Identifier(table_name)) = self.data.get("Name") {
            match database.delete_table(table_name) {
                Ok(_table) => Ok(QuerySuccess::new()),
                Err(err) => Err(err),
            }
        } else {
            Err(DbErr::Generic(String::from("Bad name param")))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::Database, queries::query_parser};

    #[test]
    fn delete_table_valid() {
        let mut database = Database::new();
        database.create_table("users", Vec::new()).unwrap();

        assert!(query_parser::execute_query(&mut database, "DELETE TABLE users;").is_ok());
    }

    #[test]
    fn delete_nonexistant_table() {
        let mut database = Database::new();
        assert!(query_parser::execute_query(&mut database, "DELETE TABLE users;").is_err());
    }
}
