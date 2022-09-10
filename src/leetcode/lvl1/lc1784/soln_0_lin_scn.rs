/// @author: Leon
/// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        const ONE: char = '1';
        const ZERO: char = '0';
        let _len_s: usize = s.len();
        let mut found_one: bool = false;
        let mut found_zero: bool = false;
        for ch in s.chars() {
            match ch {
                ONE => {
                    if found_zero {
                        return false;
                    }
                    found_one = true;
                }
                ZERO => {
                    if found_one {
                        found_zero = true;
                    }
                }
                _ => {}
            }
        }
        return true;
    }
}
