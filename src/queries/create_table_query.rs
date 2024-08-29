use std::collections::HashMap;

use crate::{database::ColumnDescription, errors::DbErr, tokenizer::Token};

use super::{Query, QuerySuccess};

pub(crate) struct CreateTableQuery {
    data: HashMap<String, Token>,
}

impl Query for CreateTableQuery {
    fn new(data: HashMap<String, Token>) -> CreateTableQuery {
        CreateTableQuery { data }
    }
    fn execute(&self, database: &mut crate::database::Database) -> Result<QuerySuccess, DbErr> {
        let table_name = self.get_table_name()?;
        let table_description = self.get_table_description()?;

        match database.create_table(table_name, table_description) {
            Ok(_table) => Ok(QuerySuccess::new()),
            Err(err) => Err(err),
        }
    }
}

impl CreateTableQuery {
    fn get_table_name(&self) -> Result<&String, DbErr> {
        if let Some(Token::Identifier(table_name)) = self.data.get("Name") {
            Ok(table_name)
        } else {
            Err(DbErr::Generic(String::from("Bad name param")))
        }
    }

    fn get_table_description(&self) -> Result<Vec<ColumnDescription>, DbErr> {
        if let Some(Token::List(tokens)) = self.data.get("TableDescription") {
            let mut i = 0;
            let mut column_descriptions: Vec<ColumnDescription> = Vec::new();

            while i < tokens.len() {
                match (&tokens[i], &tokens[i + 1]) {
                    (Token::Identifier(first), Token::Identifier(second)) => {
                        column_descriptions.push(ColumnDescription {
                            column_name: first.clone(),
                            column_type: second.clone(),
                        });

                        i += 2; // Move to the next potential series
                    }
                    _ => return Err(DbErr::Generic(String::from("Bad table description provided"))),
                }

                match tokens.get(i) {
                    None => return Ok(column_descriptions),
                    Some(Token::Comma) => i += 1,
                    _ => return Err(DbErr::Generic(String::from("Unexpected item where , or nothing was expected"))),
                }
            }
        } else {
            return Err(DbErr::Generic(String::from("Bad name param")));
        }

        Err(DbErr::Generic(String::from("Unexpected parsing error")))
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::Database, errors::DbErr, queries::query_parser};

    #[test]
    fn create_table_with_columns() {
        let mut database = Database::new();

        assert!(query_parser::execute_query(&mut database, "CREATE TABLE users (name string, age i32);").is_ok());

        let table = database.get_table("users").unwrap();
        let col1 = table.columns.get(0).unwrap();
        let col2 = table.columns.get(1).unwrap();

        assert_eq!("name", col1.column_name);
        assert_eq!("string", col1.column_type);

        assert_eq!("age", col2.column_name);
        assert_eq!("i32", col2.column_type);
    }

    #[test]
    fn create_table_twice() {
        let mut database = Database::new();
        assert!(query_parser::execute_query(&mut database, "CREATE TABLE users (name string);").is_ok());
        assert_eq!(
            DbErr::TableAlreadyExists,
            query_parser::execute_query(&mut database, "CREATE TABLE users (name string);").unwrap_err(),
        );
    }
    
}
