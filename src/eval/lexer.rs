use super::{get_function_words, get_token_operator, Token, OPERATOR_CHARS};

#[derive(Debug)]
pub struct Lexer {
    data: String,
    position: usize,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        let data = data.to_lowercase();
        Lexer { data, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.data.len() {
            return None;
        }

        let c = self.curr_char();

        self.parse_char(c)
    }

    fn parse_char(&mut self, c: char) -> Option<Token> {
        match c {
            '(' => {
                self.position += 1;
                Some(Token::ParenthesisLeft)
            }

            ')' => {
                self.position += 1;
                Some(Token::ParenthesisRight)
            }

            '0'..='9' => self.parse_number(),

            ' ' | ',' | '\n' => {
                self.position += 1;
                self.next_token()
            }

            _ => self.parse_operator(),
        }
    }

    fn curr_char(&self) -> char {
        self.data.chars().nth(self.position).unwrap_or('\0')
    }

    fn next_char(&self, offset: usize) -> char {
        self.data
            .chars()
            .nth(self.position + offset)
            .unwrap_or('\0')
    }

    fn next_chars(&self, length: usize) -> String {
        self.data.chars().skip(self.position).take(length).collect()
    }

    fn parse_number(&mut self) -> Option<Token> {
        let mut number = String::new();
        let mut c = self.curr_char();

        while c.is_ascii_digit() || c == '.' {
            number.push(c);
            self.position += 1;
            c = self.data.chars().nth(self.position).unwrap_or('\0');
        }

        let number = number.parse::<f64>();

        match number {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => None,
        }
    }

    fn parse_operator(&mut self) -> Option<Token> {
        let c = self.curr_char();

        match OPERATOR_CHARS.contains(&c) {
            true => {
                self.position += 1;
                get_token_operator(c)
            }
            false => self.parse_words(),
        }
    }

    fn parse_words(&mut self) -> Option<Token> {
        let c = self.curr_char();

        if c == 'p' && self.next_char(1) == 'i' {
            self.position += 2;
            return Some(Token::Number(std::f64::consts::PI));
        }

        if c == 'e' {
            self.position += 1;
            return Some(Token::Number(std::f64::consts::E));
        }

        for (key, operator) in get_function_words() {
            if self.next_chars(key.len()) == key {
                self.position += key.len();
                return Some(Token::Op(operator));
            }
        }

        None
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
