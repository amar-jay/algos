use crate::solution::data_stuctures::arrays::{stack::Stack, queue::Queue};
/**
* Construct a function that, when given a string containing an expression in infix notation, will return an identical expression in postfix notation.

The operators used will be +, -, *, /, and ^ with left-associativity of all operators but ^.

The precedence of the operators (most important to least) are : 1) parentheses, 2) exponentiation, 3) times/divide, 4) plus/minus.

The operands will be single-digit integers between 0 and 9, inclusive.

Parentheses may be included in the input, and are guaranteed to be in correct pairs.

toPostfix("2+7*5"); // Should return "275*+"
toPostfix("3*3/(7+1)"); // Should return "33*71+/"
toPostfix("5+(6-2)*9+3^(7-1)"); // Should return "562-9*+371-^+"
toPostfix("1^2^3"); // Should return "123^^"
You may read more about postfix notation, also called Reverse Polish notation, here: http://en.wikipedia.org/wiki/Reverse_Polish_notation
*
*
*/
#[allow(unused)]
fn to_postfix(infix: &str) -> String {
    let mut stack:Stack<char> = Stack::new();
    let mut queue:Queue<char> = Queue::new();
    let mut ans = String::from("");
    let operators = ['+','-', '*', '/', '^'];
    let mut open_parens = false;
    let mut last_operator = None;


        let mut append_to_ans = |queue: &mut Queue<char>, stack: &mut Stack<char>, ans: &mut String| {
           while let Some(el) =  queue.remove() {
                ans.push(el);
            }
            
            while let Some(el) = stack.pop() {
                ans.push(el);
            }
        };

    for i in infix.chars() {
        if i.is_numeric(){
            queue.add(i);
        } 
        if operators.contains(&i) {
            stack.push(i);
        }
    

        if i == '(' {
            open_parens = true;
            last_operator = stack.pop();
            append_to_ans(&mut queue, &mut stack, &mut ans);
        }
        if i == ')' && open_parens == true {
            append_to_ans(&mut queue, &mut stack, &mut ans);
            if let Some(op) = last_operator {
                ans.push(op);
            }
            last_operator = None;
        }
    }
            append_to_ans(&mut queue, &mut stack, &mut ans);
    ans
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_postfix;

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(actual, expected, "\nYour answer (left: {:?}) is not the correct answer (right: {:?})", actual, expected)
    }
    
    #[test]
    fn basic_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("1^2^3"), "123^^");
    }

    #[test]
    fn advanced_tests() {
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
    }
}
