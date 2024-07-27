pub(crate) mod create_table_query;
pub(crate) mod delete_table_query;
pub(crate) mod parse_steps;
pub(crate) mod query_builder;
pub(crate) mod query_parser;

use std::collections::HashMap;

use crate::{database::Database, errors::DbErr, tokenizer::Token};

pub(crate) struct QuerySuccess {}

impl QuerySuccess {
    pub fn new() -> QuerySuccess {
        QuerySuccess {}
    }
}

pub(crate) trait Query {
    fn new(data: HashMap<String, Token>) -> Self
    where
        Self: Sized;
    fn execute(&self, database: &mut Database) -> Result<QuerySuccess, DbErr>;
}
