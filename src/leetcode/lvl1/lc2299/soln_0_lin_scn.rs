use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/strong-password-checker-ii/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let len_s: usize = password.len();
        if len_s < 8 {
            return false;
        }
        let chs: Vec<char> = password.chars().collect();
        let special_chs: HashSet<char> = "!@#$%^&*()-+".chars().into_iter().collect();
        let mut at_least_one_lower: bool = false;
        let mut at_least_one_upper: bool = false;
        let mut at_least_one_digit: bool = false;
        let mut at_least_one_special: bool = false;
        for (idx, &ch) in chs.iter().enumerate() {
            if idx > 0 {
                if chs[idx - 1] == chs[idx] {
                    return false;
                }
            }
            if ch >= 'a' && ch <= 'z' {
                at_least_one_lower = true;
            }
            if ch >= 'A' && ch <= 'Z' {
                at_least_one_upper = true;
            }
            if ch >= '0' && ch <= '9' {
                at_least_one_digit = true;
            }
            if special_chs.contains(&ch) {
                at_least_one_special = true;
            }
        }
        at_least_one_lower && at_least_one_upper && at_least_one_digit && at_least_one_special
    }
}
