/*
14. Longest Common Prefix
Easy

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Note:

All given inputs are in lowercase letters a-z.
*/

fn longest_common_prefix(strs: Vec<String>) -> String {

    let mut i = 0;

    let mut prefix: String = String::from("");

    let chars: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();

    loop {
        let mut current = None;

        for cs in chars.iter() {
            if i >= cs.len() { return prefix.to_string(); }

            let c = cs[i];

            match current {
                None => current = Some(c),
                Some(v) if v != c => return prefix.to_string(),
                _ => ()
            }
        }
        prefix.push(current.unwrap());
        i = i + 1;
    }
}

#[test]
fn longest_common_prefix_test() {
    assert_eq!(longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string());
    assert_eq!(longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]), "".to_string());
}