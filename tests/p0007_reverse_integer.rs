/*
Reverse Integer - easy

Given a 32-bit signed integer, reverse digits of an integer.

Example 1:

Input: 123
Output: 321

Example 2:

Input: -123
Output: -321

Example 3:

Input: 120
Output: 21

Note:
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
*/

fn reverse(x: i32) -> i32 {
    fn reverse_impl(x: i32, state: i32) -> i32 {
        match x {
            0 => state,
            x => {
                match state.checked_mul(10).and_then(|s| s.checked_add(x % 10)) {
                    Some(s) => reverse_impl(x / 10, s),
                    None => 0
                }
            }
        }
    }

    reverse_impl(x, 0)
}

#[test]
fn reverse_integer_test() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(2_147_483_647), 0)
}