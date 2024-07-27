use crate::{database::Database, queries::query_parser::execute_query};

mod database;
mod errors;
mod queries;
mod tokenizer;

fn main() {
    let mut database = Database::new();
    let _ = execute_query(&mut database, "My query");
}
