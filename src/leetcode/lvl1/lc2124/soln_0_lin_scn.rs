/// @author: Leon
/// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_string(s: String) -> bool {
        let _len_s: usize = s.len();
        let mut met = false;
        const A: char = 'a';
        const B: char = 'b';
        for ch in s.chars() {
            match ch {
                A => {
                    if met {
                        return false;
                    }
                }
                B => met = true,
                _ => {}
            }
        }
        true
    }
}
