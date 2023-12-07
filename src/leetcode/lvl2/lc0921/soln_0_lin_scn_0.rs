/// @author: Leon
/// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        let _len_s: usize = s.len();
        let mut ans: i32 = 0;
        let mut stk: i32 = 0;
        for ch in s.chars() {
            match ch {
                PAREN_OPEN => {
                    if stk < 0 {
                        ans += stk.abs();
                        stk = 0;
                    }
                    stk += 1;
                }
                PAREN_CLOSED => stk -= 1,
                _ => {}
            }
        }
        return ans + stk.abs();
    }
}
