use std::cell::RefCell;

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
        loop {
            let current_char_option = iterator.next();
            match current_char_option {
                Some(current_char) => {
                    if current_char.is_whitespace() {
                        continue;
                    } else {
                        match current_char {
                            ',' => { return Ok(RefCell::new(Token::Comma)) },
                            '[' => { return Ok(RefCell::new(Token::SquareBracketOpen)) },
                            ']' => { return Ok(RefCell::new(Token::SquareBracketClose)) },
                            _ => {
                                if current_char.is_alphabetic() {
                                    let mut item = String::new().to_owned();
                                    item.push_str(&current_char.to_string());
                                    while let Some(inner_char) = iterator.peek() {
                                        if inner_char.is_alphabetic() {
                                            item.push_str(&inner_char.to_string());
                                            iterator.next();
                                        } else {
                                            break
                                        }
                                    }
                                    return Ok(RefCell::new(Token::ListItem { payload: RefCell::new(item) }));
                                } else {
                                    return Err(format!("Invalid character: '{}'", current_char));
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
        match next_token() {
            Ok(token_ref) => {
                let token = token_ref.into_inner();
                tokens.push(RefCell::new(token.clone()));
                match token {
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

let tokens = list_tokenizer("").unwrap().into_iter().map(|r| r.into_inner()).collect::<Vec<_>>();
assert!(matches!(tokens[0], Token::EOF));

let tokens = list_tokenizer(";");
assert!(matches!(tokens, Err(_)));

let tokens = list_tokenizer("[]").unwrap().into_iter().map(|r| r.into_inner()).collect::<Vec<_>>();
assert!(matches!(tokens[0], Token::SquareBracketOpen));
assert!(matches!(tokens[1], Token::SquareBracketClose));
assert!(matches!(tokens[2], Token::EOF));

let tokens = list_tokenizer("foo").unwrap().into_iter().map(|r| r.into_inner()).collect::<Vec<_>>();
//assert!(matches!(tokens[0], Token::ListItem { payload: RefCell::new(String::from("foo")) })); // fn calls are not allowed in patterns
let foo = RefCell::new(String::from("foo"));
assert!(matches!(&tokens[0], Token::ListItem { payload: foo }));
assert!(matches!(tokens[1], Token::EOF));

let tokens = list_tokenizer("[foo]").unwrap().into_iter().map(|r| r.into_inner()).collect::<Vec<_>>();
let foo = RefCell::new(String::from("foo"));
assert!(matches!(tokens[0], Token::SquareBracketOpen));
assert!(matches!(&tokens[1], Token::ListItem { payload: foo }));
assert!(matches!(tokens[2], Token::SquareBracketClose));
assert!(matches!(tokens[3], Token::EOF));

let tokens = list_tokenizer("[foo, bar]").unwrap().into_iter().map(|r| r.into_inner()).collect::<Vec<_>>();
let foo = RefCell::new(String::from("foo"));
let bar = RefCell::new(String::from("bar"));
assert!(matches!(tokens[0], Token::SquareBracketOpen));
assert!(matches!(&tokens[1], Token::ListItem { payload: foo }));
assert!(matches!(tokens[2], Token::Comma));
assert!(matches!(&tokens[3], Token::ListItem { payload: bar }));
assert!(matches!(tokens[4], Token::SquareBracketClose));
assert!(matches!(tokens[5], Token::EOF));
