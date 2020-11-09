fn my_atoi(s: String) -> i32 {
    if s.len() == 0 { return 0 }

    let chars: Vec<char> = s.chars().collect();

    let mut i: usize = 0;
    let mut sign: i32 = 1;
    let mut result: i32 = 0;

    // discard empty characters
    while i < chars.len() && chars[i] == ' ' {
        i += 1;
    }

    // get sign if it exists
    if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
        if chars[i] == '-' { sign = -1; }
        i += 1;
    }

    // get digits
    while i < chars.len() && chars[i].is_digit(10) {
        match result.checked_mul(10).and_then(|v| v.checked_add(chars[i].to_digit(10).unwrap() as i32)) {
            Some(r) => result = r,
            None => if sign == 1 { return std::i32::MAX } else { return std::i32::MIN }
        }

        i += 1;
    }

    result * sign
}

#[test]
fn my_atoi_test() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), i32::MIN);
    assert_eq!(my_atoi("10923098972799".to_string()), i32::MAX);
    assert_eq!(my_atoi("+42".to_string()), 42);
    assert_eq!(my_atoi("+-42".to_string()), 0);
}