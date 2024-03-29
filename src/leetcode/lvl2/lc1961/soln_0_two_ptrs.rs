/// @author: Leon
/// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/
/// Time Complexity:    O(minOf(`len_s`, 'len_ws' * ave_len_word))
/// Space Complexity:   O(ave_len_word)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let len_ws = words.len();
        let mut idx_ws: usize = 0;
        let mut idx_ch: usize = 0;
        for ch in s.chars() {
            if idx_ws >= len_ws {
                return false;
            }
            let word = &words[idx_ws];
            let len_w = word.len();
            let chs: Vec<char> = word.chars().collect();
            if ch != chs[idx_ch] {
                return false;
            }
            idx_ch += 1;
            if idx_ch == len_w {
                idx_ws += 1;
                idx_ch = 0;
            }
        }
        idx_ch == 0
    }
}
