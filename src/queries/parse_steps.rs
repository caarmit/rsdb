use std::{
    collections::HashMap,
    mem::{self},
};

use crate::{errors::DbErr, tokenizer::Token};

enum Step {
    UnnamedStep(Token),
    NamedStep(Token, String),
    NamedStream(Token, Token, String),
}

pub(crate) struct ParseSteps {
    steps: Vec<Step>,
}

impl ParseSteps {
    pub(crate) fn new() -> ParseSteps {
        ParseSteps { steps: Vec::new() }
    }

    pub(crate) fn add_named_step(mut self, token: Token, name: &str) -> Self {
        self.steps.push(Step::NamedStep(token, name.to_owned()));
        self
    }

    pub(crate) fn add_step(mut self, token: Token) -> Self {
        self.steps.push(Step::UnnamedStep(token));
        self
    }

    pub(crate) fn add_token_capture_stream(mut self, start_token: Token, end_token: Token, step_name: String) -> Self {
        self.steps.push(Step::NamedStream(start_token, end_token, step_name));
        self
    }

    pub(crate) fn parse(&self, tokens: &Vec<Token>) -> Result<HashMap<String, Token>, (DbErr, usize)> {
        let mut map: HashMap<String, Token> = HashMap::new();
        let mut token_idx = 0;

        for val in self.steps.iter() {
            let token = tokens.get(token_idx);
            if token.is_none() {
                return Err((DbErr::Generic(String::from("Unexpected end of token stream")), token_idx));
            }

            let token = token.unwrap();

            token_idx += 1;

            match val {
                Step::UnnamedStep(step_token) => {
                    if mem::discriminant(step_token) != mem::discriminant(token) {
                        return Err((
                            DbErr::Generic(format!("Unexpected token, got {:?}, expected {:?}", token, step_token,)),
                            token_idx,
                        ));
                    }
                }
                Step::NamedStep(step_token, step_name) => {
                    if mem::discriminant(step_token) != mem::discriminant(token) {
                        return Err((
                            DbErr::Generic(format!("Unexpected token, got {:?}, expected {:?}", token, step_token,)),
                            token_idx,
                        ));
                    }

                    map.insert(step_name.clone(), token.clone());
                }
                Step::NamedStream(start_token, end_token, step_name) => {
                    if mem::discriminant(token) != mem::discriminant(start_token) {
                        return Err((
                            DbErr::Generic(format!("Unexpected token, got {:?}, expected {:?}", token, start_token,)),
                            token_idx,
                        ));
                    }

                    let mut found_delimiters = false;
                    let mut token_stream: Vec<Token> = Vec::new();
                    while let Some(token) = tokens.get(token_idx) {
                        token_idx += 1;

                        if mem::discriminant(token) == mem::discriminant(end_token) {
                            found_delimiters = true;
                            break;
                        } else {
                            token_stream.push(token.clone());
                        }
                    }

                    if found_delimiters {
                        map.insert(step_name.clone(), Token::List(token_stream));
                    }
                }
            }
        }

        Ok(map)
    }
}
