mod lexer;
mod shunting_yard;
mod tokens;

use std::collections::VecDeque;

use lexer::*;
use shunting_yard::*;
use tokens::*;

pub fn eval(input: String) -> Result<f64, ()> {
    let lexer = Lexer::new(input);
    let infinix_notation = shunting_yard(lexer);
    let result_buff = evaluate_rpn(infinix_notation?)?;

    if result_buff.len() != 1 {
        return Err(());
    }

    Ok(result_buff[0])
}

fn evaluate_rpn(rpn: VecDeque<Token>) -> Result<Vec<f64>, ()> {
    let mut numbers: Vec<f64> = Vec::new();

    for token in rpn.iter() {
        let res = match token {
            Token::Number(num) => *num,
            Token::Op(op) => {
                let r = numbers.pop();
                let l = numbers.pop();
                evaluate_operator(op, l, r)?
            }
            _ => return Err(()),
        };
        numbers.push(res);
    }

    Ok(numbers)
}

fn evaluate_operator(op: &Operator, l: Option<f64>, r: Option<f64>) -> Result<f64, ()> {
    if op.number_of_operands == 1 {
        let r = r.ok_or(())?;

        let res = match op.op {
            OperatorType::Sin => r.sin(),
            OperatorType::Cos => r.cos(),
            _ => return Err(()),
        };

        return Ok(res);
    }

    let l = l.ok_or(())?;
    let r = r.ok_or(())?;

    let res = match op.op {
        OperatorType::Add => l + r,
        OperatorType::Subtract => l - r,
        OperatorType::Multiply => l * r,
        OperatorType::Divide => {
            if r == 0.0 {
                return Err(());
            }
            l / r
        }
        OperatorType::Exponent => l.powf(r),
        OperatorType::Modulus => l % r,
        OperatorType::Max => l.max(r),
        _ => return Err(()),
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_addition() {
        let input = "2 + 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn eval_subtraction() {
        let input = "2 - 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn eval_multiplication() {
        let input = "2 * 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn eval_division() {
        let input = "2 / 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn eval_division_by_zero() {
        let input = "2 / 0".to_string();
        let result = eval(input);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn eval_exponentiation() {
        let input = "2 ^ 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn eval_modulus() {
        let input = "2 % 2".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn eval_max_function() {
        let input = "max(2, 3)".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn eval_sine_function() {
        let input = "sin(0)".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn eval_cosine_function() {
        let input = "cos(0)".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn eval_invalid_input() {
        let input = "2 +".to_string();
        let result = eval(input);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn eval_invalid_statement() {
        let input = "exit".to_string();
        let result = eval(input);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn eval_example_1() {
        let input = "3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(3.0001220703125));
    }

    #[test]
    fn eval_example_2() {
        let input = "sin ( max ( 2, 3 ) ÷ 3 × pi )".to_string();
        let result = eval(input);
        assert_eq!(result, Ok(0.00000000000000012246467991473532));
    }
}
