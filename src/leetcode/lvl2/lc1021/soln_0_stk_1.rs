use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/remove-outermost-parentheses/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let _len_s: usize = s.len();
        const OPEN: char = '(';
        const CLOSED: char = ')';
        let mut stk: VecDeque<char> = VecDeque::new();
        let mut ans: String = "".to_owned();
        for ch in s.chars() {
            match ch {
                OPEN => {
                    stk.push_back(ch);
                    if stk.len() > 1 {
                        ans.push(ch);
                    }
                }
                CLOSED => {
                    stk.pop_back();
                    if !stk.is_empty() {
                        ans.push(ch);
                    }
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}
