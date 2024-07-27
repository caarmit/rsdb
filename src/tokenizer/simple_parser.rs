use super::{Token, TokenParser};

pub(crate) struct SimpleParser {
    search_string: String,
    result_token: Token,
}

impl SimpleParser {
    pub fn new(search_string: String, result_token: Token) -> SimpleParser {
        SimpleParser {
            search_string,
            result_token,
        }
    }
}

impl TokenParser for SimpleParser {
    fn parse_skip(&self, input: &mut String) -> Option<Token> {
        if input.to_lowercase().starts_with(&self.search_string) {
            input.replace_range(0..self.search_string.len(), "");
            Some(self.result_token.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{simple_parser::SimpleParser, Token, TokenParser};

    #[test]
    fn match_create() {
        let parser = SimpleParser::new(String::from("create"), Token::Create);

        let mut test1 = String::from("create EGGS");
        let mut test2 = String::from("CREATE something");
        let mut test3 = String::from("CREATE");

        assert_eq!(Some(Token::Create), parser.parse_skip(&mut test1));
        assert_eq!(Some(Token::Create), parser.parse_skip(&mut test2));
        assert_eq!(Some(Token::Create), parser.parse_skip(&mut test3));
    }

    #[test]
    fn no_matches() {
        let parser = SimpleParser::new(String::from("create"), Token::Create);

        let mut test1 = String::from("12create");
        let mut test2 = String::from("12create12");
        let mut test3 = String::from("12CREATE1");

        assert_eq!(None, parser.parse_skip(&mut test1));
        assert_eq!(None, parser.parse_skip(&mut test2));
        assert_eq!(None, parser.parse_skip(&mut test3));
    }
}
