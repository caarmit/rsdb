use crate::{database::Database, errors::DbErr, tokenizer::Tokenizer};

use super::{query_builder, Query};

pub fn execute_query(database: &mut Database, query: &str) -> Result<String, DbErr> {
    let result = create_query_plan(database, query);

    match result {
        Ok(query_plan) => {
            for query in query_plan {
                query.execute(database)?;
            }

            Ok("Done".into())
        }
        Err(_) => Err(DbErr::Generic("Not done yet".into())),
    }
}

fn create_query_plan(_database: &mut Database, query: &str) -> Result<Vec<Box<dyn Query>>, DbErr> {
    let tokenizer = Tokenizer::new();

    let query_builders = query_builder::get_builders();
    let mut query_plan: Vec<Box<dyn Query>> = Vec::new();

    let parsed_tokens = tokenizer.tokenize(query).map_err(|_| DbErr::Generic(String::from("test")))?;

    let mut best_progress: usize = 0;
    let mut best_error: DbErr = DbErr::Generic(String::from("Unknown error"));

    for builder in query_builders {
        match builder.steps.parse(&parsed_tokens) {
            Ok(parsed) => {
                let factory = builder.factory;
                query_plan.push(factory(parsed));
            }
            Err((error, progress)) => {
                if progress > best_progress {
                    best_error = error;
                    best_progress = progress;
                }
            }
        }
    }

    if query_plan.is_empty() {
        return Err(best_error);
    }

    Ok(query_plan)
}

#[cfg(test)]
mod tests {
    use crate::database::Database;
    use crate::queries::query_parser::execute_query;

    #[test]
    fn test_create_parser() {
        let mut database = Database::new();
        let result = execute_query(&mut database, "CREATE TABLE apple (age INTEGER);");
        let table = database.get_table("apple");

        assert!(result.is_ok());
        assert!(table.is_some());
        assert_eq!(String::from("age"), table.unwrap().columns[0].column_name);
        assert_eq!(String::from("INTEGER"), table.unwrap().columns[0].column_type);
    }

    #[test]
    fn test_create_parser_multiple_columns() {
        let mut database = Database::new();
        let result = execute_query(&mut database, "CREATE TABLE apple (name STRING, age INTEGER);");
        let table = database.get_table("apple");

        assert!(result.is_ok());
        assert!(table.is_some());

        assert_eq!(String::from("name"), table.unwrap().columns[0].column_name);
        assert_eq!(String::from("STRING"), table.unwrap().columns[0].column_type);

        assert_eq!(String::from("age"), table.unwrap().columns[1].column_name);
        assert_eq!(String::from("INTEGER"), table.unwrap().columns[1].column_type);
    }

    #[test]
    fn test_create_parser_errors() {
        let mut database = Database::new();

        assert!(execute_query(&mut database, "CREATE TABLE apple (;").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple ();").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple (something one two);").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple (, one, two);").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple (, one, two_));").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple (one two));").is_err());
        assert!(execute_query(&mut database, "CREATE TABLE apple ((one two);").is_err());
    }

    #[test]
    fn test_delete_parser() {
        let mut database = Database::new();
        let _ = database.create_table("apple", Vec::new());
        let _ = execute_query(&mut database, "DELETE TABLE apple");

        let table = database.get_table("apple");

        assert!(table.is_none());
    }
}
