struct TokenType {
    name: String,
}

struct Token<'a> {
    type_id: &'a TokenType,
    data: Option<String>,
}

trait Tokenizer {
    fn next_token(&mut self) -> Option<Token>;
    fn tokenize(&self) -> Vec<Token>;
}

struct ListTokenizer<'a> {
    input: &'a str,
    token_types: Vec<TokenType>,
    iterator: Box<Iterator<Item = char>>,
    current_char_result: Option<char>,
}

impl ListTokenizer {
    pub fn new(&mut self, input: &str) -> ListTokenizer {
        ListTokenizer {
            input,
            token_types: vec![
                TokenType { name: String::from("LIST_ITEM") },
                TokenType { name: String::from("COMMA") },
                TokenType { name: String::from("BRACKET_OPEN") },
                TokenType { name: String::from("BRACKET_CLOSE") },
            ],
            iterator: Box::new(input.chars()),
            current_char_result: self.iterator.next(),
        }
    }
}

impl Tokenizer for ListTokenizer {
    fn next_token(&mut self) -> Option<Token> {
        loop {
            match self.current_char_result {
                Some(current_char) => {
                    match current_char {
                        ' ' | '\t' | '\n' | '\r' => {
                            self.current_char_result = self.iterator.next();
                        },
                        ',' => {
                            Token { type_id: &self.token_types[1], data: None }
                        },
                        '[' => {
                            Token { type_id: &self.token_types[2], data: None }
                        },
                        ']' => {
                            Token { type_id: &self.token_types[3], data: None }
                        },
                    }
                },
                _ => {
                    break None
                }
            }
        }
    }
    fn tokenize(&self) -> Vec<Token> {
        let tokens = vec![];
        while let token = self.next_token() {
            tokens.push(token);
        }
        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tokenizer() {
        let input = String::from("[a, b]");
        let tokenizer = ListTokenizer(&input);
        let tokens = tokenzier.tokenize();
    }
}
