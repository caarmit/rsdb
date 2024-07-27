use std::collections::HashMap;

use crate::tokenizer::Token;

use super::{create_table_query::CreateTableQuery, delete_table_query::DeleteTableQuery, parse_steps::ParseSteps, Query};

type QueryFactory = Box<dyn Fn(HashMap<String, Token>) -> Box<dyn Query>>;

pub struct QueryBuilder {
    pub steps: ParseSteps,
    pub factory: QueryFactory,
}

pub fn get_builders() -> Vec<QueryBuilder> {
    let mut query_factories: Vec<QueryBuilder> = Vec::new();

    query_factories.push(QueryBuilder {
        steps: ParseSteps::new()
            .add_step(Token::Create)
            .add_step(Token::Table)
            .add_named_step(Token::Identifier("_".into()), "Name")
            .add_token_capture_stream(Token::OpenBracket, Token::CloseBracket, String::from("TableDescription"))
            .add_step(Token::SemiColon),
        factory: Box::new(|data| Box::new(CreateTableQuery::new(data))),
    });

    query_factories.push(QueryBuilder {
        steps: ParseSteps::new()
            .add_step(Token::Delete)
            .add_step(Token::Table)
            .add_named_step(Token::Identifier("_".into()), "Name"),
        factory: Box::new(|data| Box::new(DeleteTableQuery::new(data))),
    });

    query_factories
}
