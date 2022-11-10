/// @author: Leon
/// https://leetcode.com/problems/detect-capital/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let _len_s: usize = word.len();
        let mut seen_lower_case: bool = false;
        let mut should_all_be_capitals: bool = false;
        for (idx, ch) in word.chars().into_iter().enumerate() {
            if ch.is_ascii_uppercase() {
                if idx == 0 {
                    continue;
                } else if seen_lower_case {
                    return false;
                } else {
                    should_all_be_capitals = true;
                }
            } else {
                if should_all_be_capitals {
                    return false;
                }
                seen_lower_case = true;
            }
        }
        return true;
    }
}
