use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s` * `k`)
/// Reference:
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/discuss/1106203/Rust-Set-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let len_s: usize = s.len();
        let mut seen: HashSet<&str> = HashSet::new();
        let len: usize = std::cmp::min(len_s, k as usize);
        for i in 0..len_s - len + 1 {
            seen.insert(&s[i..i + len]);
        }
        return seen.len() == (1 << k);
    }
}
