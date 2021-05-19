/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
*/


pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' if stack.pop() != Some('(') => return false,
            ']' if stack.pop() != Some('[') => return false,
            '}' if stack.pop() != Some('{') => return false,
            _ => ()
        }
    }

    return stack.len() == 0
}

#[test]
fn is_valid_test() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("([)]".to_string()), false);
    assert_eq!(is_valid("{[]}".to_string()), true);
    assert_eq!(is_valid("[".to_string()), false);
}

