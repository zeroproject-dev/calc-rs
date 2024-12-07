use std::collections::VecDeque;

use super::{Associativity, Lexer, Operator, Token};

pub fn shunting_yard(lexer: Lexer) -> Result<VecDeque<Token>, ()> {
    let mut out: VecDeque<Token> = VecDeque::new();
    let mut operators: VecDeque<Token> = VecDeque::new();

    for token in lexer {
        match &token {
            Token::Number(_) => out.push_back(token.clone()),
            Token::ParenthesisLeft => operators.push_front(token.clone()),
            Token::ParenthesisRight => parse_parenthesis(&mut operators, &mut out)?,
            Token::Op(operator) => parse_operator(&token, operator, &mut operators, &mut out)?,
        }
    }

    operators.into_iter().for_each(|token| {
        out.push_back(token);
    });

    Ok(out)
}

fn parse_parenthesis(operators: &mut VecDeque<Token>, out: &mut VecDeque<Token>) -> Result<(), ()> {
    while let Some(token) = operators.pop_front() {
        if token != Token::ParenthesisLeft {
            out.push_back(token);
            continue;
        }

        if let Some(Token::Op(op)) = operators.front() {
            if op.precedence == 5 {
                let front = operators.pop_front().unwrap();
                out.push_back(front);
            }
        }

        return Ok(());
    }

    Err(())
}

fn parse_operator(
    token: &Token,
    operator: &Operator,
    operators: &mut VecDeque<Token>,
    out: &mut VecDeque<Token>,
) -> Result<(), ()> {
    while let Some(Token::Op(op)) = operators.front() {
        if op.precedence != operator.precedence || operator.associativity == Associativity::Right {
            break;
        }

        out.push_back(operators.pop_front().unwrap());
    }

    operators.push_front(token.clone());
    Ok(())
}
