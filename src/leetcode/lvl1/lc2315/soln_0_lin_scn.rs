/// @author: Leon
/// https://leetcode.com/problems/count-asterisks/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let _len_s: usize = s.len();
        let mut stk: u8 = 0;
        let mut cnt: i32 = 0;
        const PIPE: u8 = b'|';
        const ASTERISK: u8 = b'*';
        for &b in s.as_bytes() {
            match b {
                PIPE => {
                    stk ^= 1;
                }
                ASTERISK => {
                    if stk == 0 {
                        cnt += 1;
                    }
                }
                _ => {}
            }
        }
        cnt
    }
}
