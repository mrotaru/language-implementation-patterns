#![allow(unused_variables)]
use std::cell::RefCell;
use std::error::Error;

#[derive(Debug, Clone)]
enum Token {
    Comma,
    SquareBracketOpen,
    SquareBracketClose,
    ListItem { payload: RefCell<String> },
    EOF,
}

fn list_tokenizer (input: &str) -> Result<Vec<RefCell<Token>>, String> {
    let mut iterator = input.chars().peekable();
    let mut tokens = vec![];

    let mut next_token = || -> Result<RefCell<Token>, String> {
        let mut inside_item = false;
        loop {
            let mut current_char_option = iterator.next();
            match current_char_option {
                Some(c) => {
                    if should_be_ignored(&c) {
                        continue;
                    } else {
                        match c {
                            ',' => { return Ok(RefCell::new(Token::Comma)) },
                            '[' => { return Ok(RefCell::new(Token::SquareBracketOpen)) },
                            ']' => { return Ok(RefCell::new(Token::SquareBracketClose)) },
                            _ => {
                                if c.is_alphabetic() {
                                    let mut item = String::new().to_owned();
                                    item.push_str(&c.to_string());
                                    while let Some(inner_c) = iterator.peek() {
                                        if inner_c.is_alphabetic() {
                                            item.push_str(&inner_c.to_string());
                                            iterator.next();
                                        } else {
                                            break
                                        }
                                    }
                                    return Ok(RefCell::new(Token::ListItem { payload: RefCell::new(item) }));
                                } else {
                                    return Err(format!("Invalid character: '{}'", c));
                                }
                            }
                        }
                    }
                },
                None => {
                    return Ok(RefCell::new(Token::EOF));
                },
            }
        }
    };

    loop {
        let token_res = next_token();
        match token_res {
            Ok(token) => {
                let inner = token.into_inner();
                let inner_clone = inner.clone();
                tokens.push(RefCell::new(inner));
                match inner_clone {
                    Token::EOF => { return Ok(tokens) },
                    _ => {
                        continue;
                    }
                };
            },
            Err(error) => { return Err(error) },
        };
    };
}

fn should_be_ignored (input: &char) -> bool {
    let ignore = vec![' ', '\t', '\n', '\r'];
    let mut iter = ignore.iter();
    let result = iter.find(|&&x| x == *input);
    return match result {
        Some(_) => true,
        _ => false
    }
}

println!("{:?}", list_tokenizer(""));
println!("{:?}", list_tokenizer(","));
println!("{:?}", list_tokenizer(";"));
println!("{:?}", list_tokenizer("[]"));
println!("{:?}", list_tokenizer("[[["));
println!("{:?}", list_tokenizer("foo"));
println!("{:?}", list_tokenizer("[foo]"));
println!("{:?}", list_tokenizer("[foo, bar]"));
