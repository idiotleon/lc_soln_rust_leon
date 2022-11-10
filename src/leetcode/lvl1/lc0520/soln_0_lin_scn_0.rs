/// @author: Leon
/// https://leetcode.com/problems/detect-capital/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let _len_s: usize = word.len();
        let mut any_lower: bool = false;
        let mut cnt_upper: i32 = 0;
        for ch in word.chars() {
            if ch >= 'A' && ch <= 'Z' {
                cnt_upper += 1;
                if any_lower {
                    return false;
                }
            } else if ch >= 'a' && ch <= 'z' {
                any_lower = true;
                if cnt_upper > 1 {
                    return false;
                }
            }
        }
        return true;
    }
}
