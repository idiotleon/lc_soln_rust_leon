/// @author: Leon
/// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
/// Time Complexity:    O(`len_ws1` * avg_len_word)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
        let len_ws1: usize = words1.len();
        let len_ws2: usize = words2.len();
        let chses1: Vec<Vec<char>> = words1.into_iter().map(|w| w.chars().collect()).collect();
        let chses2: Vec<Vec<char>> = words2.into_iter().map(|w| w.chars().collect()).collect();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut idx_ch1: usize = 0;
        let mut idx_ch2: usize = 0;
        while idx1 < len_ws1 && idx2 < len_ws2 {
            let chs1 = &chses1[idx1];
            let chs2 = &chses2[idx2];
            let len1 = chs1.len();
            let len2 = chs2.len();
            if chses1[idx1][idx_ch1] != chses2[idx2][idx_ch2] {
                return false;
            }
            idx_ch1 += 1;
            idx_ch2 += 1;
            if idx_ch1 == len1 {
                idx1 += 1;
                idx_ch1 = 0;
            }
            if idx_ch2 == len2 {
                idx2 += 1;
                idx_ch2 = 0;
            }
        }
        return idx1 == len_ws1 && idx2 == len_ws2;
    }
}
