#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    ParenthesisLeft,
    ParenthesisRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus,

    Sin,
    Cos,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub op: OperatorType,
    pub associativity: Associativity,
    pub precedence: u8,
    pub literal: String,
    pub number_of_operands: u8,
}

impl AsRef<std::ffi::OsStr> for Operator {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        <String as AsRef<std::ffi::OsStr>>::as_ref(&self.literal)
    }
}

pub const OPERATOR_CHARS: [char; 9] = ['+', '-', '−', 'x', '×', '/', '÷', '^', '%'];

pub fn get_function_words() -> Vec<(&'static str, Operator)> {
    vec![
        (
            "**",
            Operator {
                op: OperatorType::Exponent,
                associativity: Associativity::Right,
                precedence: 4,
                number_of_operands: 2,
                literal: "**".to_string(),
            },
        ),
        (
            "*",
            Operator {
                op: OperatorType::Multiply,
                associativity: Associativity::Left,
                precedence: 3,
                number_of_operands: 2,
                literal: "*".to_string(),
            },
        ),
        (
            "sin",
            Operator {
                op: OperatorType::Sin,
                associativity: Associativity::Right,
                precedence: 5,
                number_of_operands: 1,
                literal: "sin".to_string(),
            },
        ),
        (
            "cos",
            Operator {
                op: OperatorType::Cos,
                associativity: Associativity::Right,
                precedence: 5,
                number_of_operands: 1,
                literal: "cos".to_string(),
            },
        ),
        (
            "max",
            Operator {
                op: OperatorType::Max,
                associativity: Associativity::Left,
                precedence: 5,
                number_of_operands: 2,
                literal: "max".to_string(),
            },
        ),
    ]
}

pub fn get_token_operator(c: char) -> Option<Token> {
    match c {
        '+' => Some(Token::Op(Operator {
            op: OperatorType::Add,
            associativity: Associativity::Left,
            precedence: 2,
            number_of_operands: 2,
            literal: c.to_string(),
        })),
        '-' | '−' => Some(Token::Op(Operator {
            op: OperatorType::Subtract,
            associativity: Associativity::Left,
            precedence: 2,
            number_of_operands: 2,
            literal: c.to_string(),
        })),
        '/' | '÷' => Some(Token::Op(Operator {
            op: OperatorType::Divide,
            associativity: Associativity::Left,
            number_of_operands: 2,
            precedence: 3,
            literal: c.to_string(),
        })),
        'x' | '×' => Some(Token::Op(Operator {
            op: OperatorType::Multiply,
            associativity: Associativity::Left,
            precedence: 3,
            number_of_operands: 2,
            literal: c.to_string(),
        })),
        '^' => Some(Token::Op(Operator {
            op: OperatorType::Exponent,
            associativity: Associativity::Right,
            precedence: 4,
            number_of_operands: 2,
            literal: c.to_string(),
        })),
        '%' => Some(Token::Op(Operator {
            op: OperatorType::Modulus,
            associativity: Associativity::Left,
            precedence: 3,
            number_of_operands: 2,
            literal: c.to_string(),
        })),
        _ => None,
    }
}
