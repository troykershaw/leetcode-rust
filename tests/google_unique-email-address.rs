/*

*/

use std::collections::HashSet;

fn num_unique_emails (emails: Vec<String>) -> i32 {

    let mut set: HashSet<String> = HashSet::new();

    for e in emails {
        let mut split: Vec<&str> = e.split('@').collect();
        if let Some(i) = split[0].chars().position(|c| c == '+') {
            split[0] = &split[0][..i];
        }

        let no_dot = split[0].replace(".", "");
        split[0] = &no_dot;
        set.insert(split.join("@"));
    }

    set.len() as i32
}

#[test]
fn num_unique_emails_test() {
    // assert_eq!(num_unique_emails(vec!["test.email+alex@leetcode.com".to_string(),"test.e.mail+bob.cathy@leetcode.com".to_string(),"testemail+david@lee.tcode.com".to_string()]), 2);
    assert_eq!(num_unique_emails(vec!["test.email+alex@leetcode.com".to_string(),"test.email.leet+alex@code.com".to_string()]), 2);
}
