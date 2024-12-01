use std::{
    collections::VecDeque,
    io::{self, Write},
};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Op(Operator),
    ParenthesisLeft,
    ParenthesisRight,
    //TODO: Check for more tokens
}

#[derive(Debug, Clone, Copy)]
enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus,
}

#[derive(Debug, Clone, Copy)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Operator {
    op: OperatorType,
    associativity: Associativity,
    precedence: u8,
    literal: String,
}

impl AsRef<std::ffi::OsStr> for Operator {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        <String as AsRef<std::ffi::OsStr>>::as_ref(&self.literal)
    }
}

#[derive(Debug)]
struct Lexer {
    data: String,
    position: usize,
}

impl Lexer {
    fn new(data: String) -> Self {
        Lexer { data, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
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

            ' ' => {
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

        match c {
            '+' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Add,
                    associativity: Associativity::Left,
                    precedence: 2,
                    literal: c.to_string(),
                }))
            }

            '-' | '−' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Subtract,
                    associativity: Associativity::Left,
                    precedence: 2,
                    literal: c.to_string(),
                }))
            }

            '/' | '÷' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Divide,
                    associativity: Associativity::Left,
                    precedence: 3,
                    literal: c.to_string(),
                }))
            }

            'x' | '×' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Multiply,
                    associativity: Associativity::Left,
                    precedence: 3,
                    literal: c.to_string(),
                }))
            }

            '^' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Exponent,
                    associativity: Associativity::Right,
                    precedence: 4,
                    literal: c.to_string(),
                }))
            }

            '%' => {
                self.position += 1;
                Some(Token::Op(Operator {
                    op: OperatorType::Modulus,
                    associativity: Associativity::Left,
                    precedence: 3,
                    literal: c.to_string(),
                }))
            }

            '*' => {
                if self.next_char(1) == '*' {
                    self.position += 2;
                    Some(Token::Op(Operator {
                        op: OperatorType::Exponent,
                        associativity: Associativity::Right,
                        precedence: 4,
                        literal: "**".to_string(),
                    }))
                } else {
                    self.position += 1;
                    Some(Token::Op(Operator {
                        op: OperatorType::Multiply,
                        associativity: Associativity::Left,
                        precedence: 3,
                        literal: c.to_string(),
                    }))
                }
            }

            _ => None,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn shutting_yard(lexer: Lexer) -> Result<VecDeque<Token>, ()> {
    let mut out: VecDeque<Token> = VecDeque::new();
    let mut operators: VecDeque<Token> = VecDeque::new();

    for token in lexer {
        match &token {
            Token::Number(_) => out.push_back(token.clone()),
            Token::ParenthesisLeft => operators.push_front(token.clone()),
            Token::ParenthesisRight => loop {
                if let Some(op) = operators.pop_front() {
                    match op {
                        Token::ParenthesisLeft => break,
                        _ => out.push_back(op),
                    }
                } else {
                    return Err(());
                }
            },
            Token::Op(operator) => {
                if operators.is_empty() {
                    operators.push_front(token);
                    continue;
                }

                loop {
                    if let Some(last_op) = operators.front() {
                        match last_op {
                            Token::Op(op) => {
                                if op.precedence != operator.precedence {
                                    break;
                                } else {
                                    match op.associativity {
                                        Associativity::Left => {}
                                        Associativity::Right => break,
                                    }
                                }
                            }
                            _ => break,
                        }
                    } else {
                        return Err(());
                    }

                    if let Some(token) = operators.pop_front() {
                        out.push_back(token);
                    } else {
                        return Err(());
                    }
                }

                operators.push_front(token);
            }
        }
    }

    operators.into_iter().for_each(|token| {
        out.push_back(token);
    });

    Ok(out)
}

fn evaluate_rpn(rpn: VecDeque<Token>) -> Result<f64, ()> {
    let mut numbers: Vec<f64> = Vec::new();

    rpn.iter().for_each(|token| match token {
        Token::Number(num) => numbers.push(*num),
        Token::Op(op) => {
            let r = numbers.pop().unwrap();
            let l = numbers.pop().unwrap();
            numbers.push(match op.op {
                OperatorType::Add => l + r,
                OperatorType::Subtract => l - r,
                OperatorType::Multiply => l * r,
                OperatorType::Divide => l / r,
                OperatorType::Exponent => l.powf(r),
                OperatorType::Modulus => l % r,
            });
        }
        _ => {}
    });

    if numbers.len() != 1 {
        Err(())
    } else {
        Ok(numbers[0])
    }
}

fn eval(input: String) -> Result<f64, ()> {
    let lexer = Lexer::new(input);
    let infinix_notation = shutting_yard(lexer);
    evaluate_rpn(infinix_notation?)
}

fn main() {
    println!("Enter a statement:");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        if let Ok(result) = eval(input) {
            println!("{}", result);
        } else {
            println!("Invalid statement");
        }
    }
}
