use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/discuss/419402/JavaC%2B%2B-Stack
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let _len_s: usize = s.len();
        const IMPS: char = '#';
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        let mut chs: Vec<char> = s.chars().collect();
        let mut stk: VecDeque<usize> = VecDeque::new();
        for (idx, ch) in s.chars().into_iter().enumerate() {
            match ch {
                PAREN_OPEN => stk.push_back(idx),
                PAREN_CLOSED => {
                    if stk.is_empty() {
                        chs[idx] = IMPS;
                    } else {
                        stk.pop_back();
                    }
                }
                _ => {}
            };
        }
        while let Some(idx) = stk.pop_back() {
            chs[idx] = IMPS;
        }
        chs.into_iter().filter(|&c| c != IMPS).collect::<String>()
    }
}
