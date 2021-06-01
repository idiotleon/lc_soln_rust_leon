/// https://leetcode.com/problems/reverse-words-in-a-string/
///
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
///
/// Reference:
/// https://leetcode.com/problems/reverse-words-in-a-string/discuss/392048/Rust-One-liner
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}
