/// @author: Leon
/// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
/// Time Complexity:    O(max(`_len_ws1`, `_len_ws2`) * avg_len_word)
/// Space Complexity:   O(max(`_len_ws1`, `_len_ws2`) * avg_len_word)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
        let _len_ws1: usize = words1.len();
        let _len_ws2: usize = words2.len();
        let s1: String = words1.join("");
        let s2: String = words2.join("");
        return s1 == s2;
    }
}
