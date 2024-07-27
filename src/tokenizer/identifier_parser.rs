use std::collections::HashMap;

use super::{Token, TokenParser};
use regex::Regex;

pub(crate) struct IdentifierParser {
    regex: Regex,
    token_mapping: HashMap<String, Token>,
}

impl IdentifierParser {
    pub(crate) fn new() -> IdentifierParser {
        IdentifierParser {
            regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            token_mapping: HashMap::new(),
        }
    }

    pub(crate) fn add_token_mapping(&mut self, identifier: String, token_type: Token) {
        self.token_mapping
            .insert(identifier.to_lowercase(), token_type);
    }
}

impl TokenParser for IdentifierParser {
    fn parse_skip(&self, input: &mut String) -> Option<super::Token> {
        match self.regex.find(&input.clone()) {
            Some(result) => {
                input.replace_range(0..result.len(), "");

                // Check for a mapped identifier, and return that if we can
                let lower_result = result.as_str().to_lowercase();

                match self.token_mapping.get(&lower_result) {
                    Some(mapping) => Some(mapping.clone()),
                    None => Some(Token::Identifier(result.as_str().to_owned())),
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{identifier_parser::IdentifierParser, Token, TokenParser};

    #[test]
    fn string_identifier() {
        let token = IdentifierParser::new().parse_skip(&mut String::from("MYSTRINGIDENTIFIER"));

        assert_eq!(
            Token::Identifier(String::from("MYSTRINGIDENTIFIER")),
            token.unwrap()
        );
    }

    #[test]
    fn identifiers_with_underscores() {
        let token1 = IdentifierParser::new().parse_skip(&mut String::from("my_str"));
        let token2 = IdentifierParser::new().parse_skip(&mut String::from("_egg"));
        let token3 = IdentifierParser::new().parse_skip(&mut String::from("_"));

        assert_eq!(Token::Identifier(String::from("my_str")), token1.unwrap());
        assert_eq!(Token::Identifier(String::from("_egg")), token2.unwrap());
        assert_eq!(Token::Identifier(String::from("_")), token3.unwrap());
    }

    #[test]
    fn identifiers_with_numbers() {
        let token1 = IdentifierParser::new().parse_skip(&mut String::from("hi1"));
        let token2 = IdentifierParser::new().parse_skip(&mut String::from("h_32"));

        assert_eq!(Token::Identifier(String::from("hi1")), token1.unwrap());
        assert_eq!(Token::Identifier(String::from("h_32")), token2.unwrap());
    }

    #[test]
    fn with_token_mapping() {
        let mut parser = IdentifierParser::new();
        parser.add_token_mapping(String::from("Create"), Token::Create);
        parser.add_token_mapping(String::from("TABLE"), Token::Table);

        let create1 = parser.parse_skip(&mut String::from("CREATE")).unwrap();
        let create2 = parser.parse_skip(&mut String::from("CREATE EGGS")).unwrap();
        let table1 = parser.parse_skip(&mut String::from("TABLE EGGS")).unwrap();
        let table2 = parser.parse_skip(&mut String::from("table EGGS")).unwrap();

        assert_eq!(Token::Create, create1);
        assert_eq!(Token::Create, create2);
        assert_eq!(Token::Table, table1);
        assert_eq!(Token::Table, table2);
    }

    #[test]
    fn no_matches() {
        let empty1 = IdentifierParser::new().parse_skip(&mut String::from("123_hello"));
        let empty2 = IdentifierParser::new().parse_skip(&mut String::from("+932"));

        assert_eq!(None, empty1);
        assert_eq!(None, empty2);
    }

    #[test]
    fn no_matches_with_mapping() {
        let mut parser = IdentifierParser::new();
        parser.add_token_mapping(String::from("Table"), Token::Table);

        assert_eq!(
            Token::Identifier(String::from("Table_")),
            parser.parse_skip(&mut String::from("Table_")).unwrap()
        );
    }
}
