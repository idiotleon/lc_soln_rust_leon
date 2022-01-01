/// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`1`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let _len_s = s.len();
        let mut stk: i16 = 0;
        let mut cnt: i16 = 0;
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        for ch in s.chars() {
            match ch {
                PAREN_OPEN => {
                    stk += 1;
                }
                PAREN_CLOSED => {
                    stk -= 1;
                }
                _ => {}
            }
            if stk == -1 {
                cnt += 1;
                stk += 1;
            }
        }
        (cnt + stk) as i32
    }
}
