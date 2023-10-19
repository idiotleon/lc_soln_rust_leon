use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/backspace-string-compare/
/// Time Complexity:    O(`_len_s` + `_len_t`) ~ O(max(`_len_s`, `_len_t`))
/// Space Complexity:   O(max(`_len_s`, `_len_t`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let _len_s: usize = s.len();
        let _len_t: usize = t.len();
        Self::get_string(s) == Self::get_string(t)
    }
    fn get_string(s: String) -> String {
        const BACKSPACE: char = '#';
        let len_s: usize = s.len();
        let mut stk: VecDeque<char> = VecDeque::with_capacity(len_s);
        for ch in s.chars() {
            if ch == BACKSPACE {
                if !stk.is_empty() {
                    stk.pop_back();
                }
            } else {
                stk.push_back(ch);
            }
        }
        return stk.into_iter().collect();
    }
}
