use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/ternary-expression-parser/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/ternary-expression-parser/discuss/92166/Very-easy-1-pass-Stack-Solution-in-JAVA-(NO-STRING-CONCAT)/96757
/// https://leetcode.com/problems/ternary-expression-parser/discuss/92166/Very-easy-1-pass-Stack-Solution-in-JAVA-(NO-STRING-CONCAT)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let len_s: usize = expression.len();
        const TRUE: char = 'T';
        const FALSE: char = 'F';
        const TERNARY: char = '?';
        let chs: Vec<char> = expression.chars().collect();
        let mut stk: VecDeque<char> = {
            let mut stk = VecDeque::with_capacity(len_s);
            stk.push_back(chs[len_s - 1]);
            stk
        };
        for idx in (0..len_s - 2).rev().step_by(2) {
            if chs[idx + 1] == TERNARY {
                let ch_true: char = stk.pop_back().unwrap();
                let ch_false: char = stk.pop_back().unwrap();
                stk.push_back(if chs[idx] == TRUE { ch_true } else { ch_false });
            } else {
                stk.push_back(chs[idx]);
            }
        }
        stk.pop_back().unwrap().to_string()
    }
}
