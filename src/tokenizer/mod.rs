use crate::errors::DbErr;

use self::{identifier_parser::IdentifierParser, simple_parser::SimpleParser};

mod identifier_parser;
mod regex_parser;
mod simple_parser;

#[derive(Debug, PartialEq, Copy, Clone)]
struct ErrorInfo;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Create,
    Delete,
    Table,
    Comma,
    OpenBracket,
    CloseBracket,
    SemiColon,
    Identifier(String),
    List(Vec<Token>),
}

pub trait TokenParser {
    fn parse_skip(&self, input: &mut String) -> Option<Token>;
}

pub struct Tokenizer {
    parsers: Vec<Box<dyn TokenParser>>,
}

impl Tokenizer {
    pub(crate) fn new() -> Tokenizer {
        let mut identifier_parser = IdentifierParser::new();
        identifier_parser.add_token_mapping(String::from("create"), Token::Create);
        identifier_parser.add_token_mapping(String::from("table"), Token::Table);
        identifier_parser.add_token_mapping(String::from("delete"), Token::Delete);

        Tokenizer {
            parsers: vec![
                Box::from(identifier_parser),
                Box::from(SimpleParser::new(String::from(","), Token::Comma)),
                Box::from(SimpleParser::new(String::from("("), Token::OpenBracket)),
                Box::from(SimpleParser::new(String::from(")"), Token::CloseBracket)),
                Box::from(SimpleParser::new(String::from(";"), Token::SemiColon)),
            ],
        }
    }

    pub(crate) fn tokenize(&self, query_string: &str) -> Result<Vec<Token>, DbErr> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut skippable_string = query_string.trim().to_string();
        while !skippable_string.is_empty() {
            let mut success = false;

            for parser in &self.parsers {
                if let Some(result) = parser.parse_skip(&mut skippable_string) {
                    tokens.push(result);
                    success = true;
                    skippable_string = skippable_string.trim_start().to_string();
                }
            }

            if !success {
                return Err(DbErr::Generic(String::from("Unable to parse")));
            }
        }

        Ok(tokens)
    }
}
#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, Tokenizer};

    #[test]
    pub fn test_create_table() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("CREATE TABLE").unwrap();

        assert_eq!(vec![Token::Create, Token::Table], tokens);
    }

    #[test]
    pub fn test_delete_table() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("delete DELETE TABLE").unwrap();

        assert_eq!(vec![Token::Delete, Token::Delete, Token::Table], tokens);
    }
}
