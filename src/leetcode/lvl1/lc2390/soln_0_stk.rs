use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/removing-stars-from-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        const ASTERISK: char = '*';
        let len_s: usize = s.len();
        let mut stk: VecDeque<char> = VecDeque::with_capacity(len_s);
        for ch in s.chars() {
            if ch == ASTERISK {
                if !stk.is_empty() {
                    stk.pop_back();
                }
            } else {
                stk.push_back(ch);
            }
        }
        return stk.into_iter().collect::<String>();
    }
}
