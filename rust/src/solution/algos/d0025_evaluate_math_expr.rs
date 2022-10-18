use std::{num::ParseFloatError, string::ParseError, char::ParseCharError};

/**
# Instructions

Given a mathematical expression as a string you must return the result as a number.

## Numbers
Number may be both whole numbers and/or decimal numbers. The same goes for the returned result.

## Operators

You need to support the following mathematical operators:

- Multiplication *
- Division / (as floating point division)
- Addition +
- Subtraction -
- Operators are always evaluated from left-to-right, and * and / must be evaluated before + and -.

Parentheses
You need to support multiple levels of nested parentheses, ex. (2 / (2 + 3.33) * 4) - -6

Whitespace
There may or may not be whitespace between numbers and operators.

An addition to this rule is that the minus sign (-) used for negating numbers and parentheses will never be separated by whitespace. I.e all of the following are valid expressions.
```
1-1    // 0
1 -1   // 0
1- 1   // 0
1 - 1  // 0
1- -1  // 2
1 - -1 // 2
1--1   // 2

6 + -(4)   // 2
6 + -( -4) // 10
```
And the following are invalid expressions
```
1 - - 1    // Invalid
1- - 1     // Invalid
6 + - (4)  // Invalid
6 + -(- 4) // Invalid
```
## Validation

You do not need to worry about validation - you will only receive valid 
mathematical expressions following the above rules.

Restricted APIs
NOTE: std::process::Command is disallowed in your solution.


*/
///#[allow(unused)]
fn calc(expr: &str) -> f64 {
    let expr = expr.chars();
    
    enum Operations {
        Addition,
        Subtraction,
        Multiplication,
        Division,
        Error(Option<ParseFloatError>),
        Ok
    }
    // looping through the expr 
    
    let mut err = Operations::Ok;
    let sum = expr.fold(0_f64, |a, b| { 

        match b {
            ' ' => return a,
            '+' => {
                return a + b.to_string().parse().unwrap_or_else(|x| {
                    err = Operations::Error(Some(x)); 
                    0_f64
                });
            },
            '-' => {
                return a - b.to_string().parse().unwrap_or_else(|x| {
                    err = Operations::Error(Some(x)); 
                    0_f64
                });
            },
            '*' => {
                return a * b.to_string().parse().unwrap_or_else(|x| {
                  err = Operations::Error(Some(x)); 
                  0_f64
                });
            },
            '/' => {
                 return a / b.to_string().parse().unwrap_or_else(|x| {
                    err = Operations::Error(Some(x)); 
                  0_f64
                });
            },
            _ => {
                err = Operations::Error(None);
                0_f64
            }
                
        }
    });
       sum 
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        }
    }
    
    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}
