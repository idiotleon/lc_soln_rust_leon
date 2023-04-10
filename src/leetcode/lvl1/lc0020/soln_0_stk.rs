use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/valid-parentheses/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let len_s: usize = s.len();
        const PAREN_OPEN1: char = '(';
        const PAREN_OPEN2: char = '[';
        const PAREN_OPEN3: char = '{';
        const PAREN_CLOSED1: char = ')';
        const PAREN_CLOSED2: char = ']';
        const PAREN_CLOSED3: char = '}';
        let mut stk: VecDeque<char> = VecDeque::with_capacity(len_s);
        for ch in s.chars() {
            match ch {
                PAREN_OPEN1 => stk.push_back(PAREN_CLOSED1),
                PAREN_OPEN2 => stk.push_back(PAREN_CLOSED2),
                PAREN_OPEN3 => stk.push_back(PAREN_CLOSED3),
                _ => {
                    if let Some(top) = stk.pop_back() {
                        if top != ch {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        return stk.is_empty();
    }
}
