/// @author: Leon
/// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
/// Time Complexity:    O(max(`_len1`, `_len2`))
/// Space Complexity:   O(max(`_len1`, `_len2`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
        let _len_ws1: usize = words1.len();
        let _len_ws2: usize = words2.len();
        let word1 = words1.join("");
        let _len1: usize = word1.len();
        let word2 = words2.join("");
        let _len2: usize = word2.len();
        return word1 == word2;
    }
}
