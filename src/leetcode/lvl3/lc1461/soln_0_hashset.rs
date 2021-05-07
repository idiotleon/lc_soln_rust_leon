/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/discuss/1106203/Rust-Set-Solution
use std::collections::HashSet;
use std::cmp;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut seen: HashSet<&str> = HashSet::new();
        let len: usize = cmp::min(s.len(), k as usize);

        for i in 0..s.len() - len + 1 {
            seen.insert(&s[i..i + len]);
        }
        return seen.len() == (1 << k);
    }
}
