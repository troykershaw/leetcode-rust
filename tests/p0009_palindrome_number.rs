/*
9. Palindrome Number
Easy

Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

Example 1:

Input: 121
Output: true

Example 2:

Input: -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Follow up:

Coud you solve it without converting the integer to a string?
*/

fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) { return false; }

    fn go(x: i32, state: i32) -> bool {
        match x {
            x if x <= state => x == state || x == state/10,
            x => go(x/10, state*10 + x%10)
        }
    }

    go(x, 0)
}

#[test]
fn palindrome_number_test() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}