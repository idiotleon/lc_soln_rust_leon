/// @author: Leon
/// https://leetcode.com/problems/length-of-last-word/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let _len_s: usize = s.len();
        const SPACE: char = ' ';
        let mut prev: u16 = 0;
        let mut cur: u16 = 0;
        for ch in s.chars() {
            if ch == SPACE {
                if cur != 0 {
                    prev = cur;
                }
                cur = 0;
            } else {
                cur += 1;
            }
        }
        if cur == 0 {
            prev as i32
        } else {
            cur as i32
        }
    }
}
