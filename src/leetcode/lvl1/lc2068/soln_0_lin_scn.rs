/// @author: Leon
/// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/
/// Time Complexity:    O(`_len1` + `_len2`) ~ O(`_len1`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let _len1: usize = word1.len();
        let _len2: usize = word2.len();
        let mut freqs: Vec<i8> = vec![0; 26];
        for ch in word1.chars() {
            freqs[ch as usize - 'a' as usize] += 1;
        }
        for ch in word2.chars() {
            freqs[ch as usize - 'a' as usize] -= 1;
        }
        for freq in freqs {
            if freq.abs() > 3 {
                return false;
            }
        }
        true
    }
}
