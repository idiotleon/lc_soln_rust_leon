use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/score-of-parentheses/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/score-of-parentheses/discuss/141777/C%2B%2BJavaPython-O(1)-Space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let len_s: usize = s.len();
        const PAREN_OPEN: char = '(';
        let mut ans: i32 = 0;
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_s);
        for ch in s.chars() {
            if ch == PAREN_OPEN {
                stk.push_back(ans);
                ans = 0;
            } else {
                if let Some(top) = stk.pop_back() {
                    ans += top + std::cmp::max(ans, 1);
                }
            }
        }
        ans
    }
}
