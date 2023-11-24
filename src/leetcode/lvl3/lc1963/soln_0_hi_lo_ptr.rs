/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        const PAREN_OPEN: char = '[';
        const PAREN_CLOSED: char = ']';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut lo: usize = 0;
        let mut stk_lo: i32 = 0;
        let mut hi: usize = len_s - 1;
        let mut stk_hi: i32 = 0;
        let mut cnt: i32 = 0;
        while lo < hi {
            while lo < hi {
                if chs[lo] == PAREN_OPEN {
                    stk_lo += 1;
                } else {
                    stk_lo -= 1;
                }
                if stk_lo < 0 {
                    break;
                }
                lo += 1;
            }
            while lo < hi {
                if chs[hi] == PAREN_OPEN {
                    stk_hi -= 1;
                } else {
                    stk_hi += 1;
                }
                if stk_hi < 0 {
                    break;
                }
                hi -= 1;
            }
            if lo < hi {
                stk_lo += 2;
                stk_hi += 2;
                cnt += 1;
            }
            lo += 1;
            hi -= 1;
        }
        return cnt;
    }
}
