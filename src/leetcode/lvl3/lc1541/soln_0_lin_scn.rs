/// @author: Leon
/// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/solutions/780199/java-c-python-straight-forward-one-pass/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        let _len_s: usize = s.len();
        let mut open_missing: i32 = 0;
        let mut closed_needed: i32 = 0;
        let mut closed_missing: i32 = 0;
        for ch in s.chars() {
            match ch {
                PAREN_OPEN => {
                    if closed_needed % 2 != 0 {
                        closed_missing += 1;
                        closed_needed -= 1;
                    }
                    closed_needed += 2;
                }
                PAREN_CLOSED => {
                    closed_needed -= 1;
                    if closed_needed < 0 {
                        open_missing += 1;
                        closed_needed += 2;
                    }
                }
                _ => unreachable!(),
            }
        }
        return open_missing + closed_missing + closed_needed;
    }
}
