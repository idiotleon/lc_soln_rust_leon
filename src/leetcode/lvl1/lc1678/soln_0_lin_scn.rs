/// @author: Leon
/// https://leetcode.com/problems/goal-parser-interpretation/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn interpret(command: String) -> String {
        const IMPL: char = '#';
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        let _len_s: usize = command.len();
        let mut prev: char = IMPL;
        let mut ans: String = "".to_owned();
        for ch in command.chars() {
            match ch {
                PAREN_OPEN => {
                    // do nothing
                }
                PAREN_CLOSED => {
                    if prev == PAREN_OPEN {
                        ans.push('o');
                    }
                }
                _ => {
                    ans.push(ch);
                }
            }
            prev = ch;
        }
        return ans;
    }
}
