use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/basic-calculator/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        const SPACE: char = ' ';
        const SIGN_PLUS: char = '+';
        const SIGN_MINUS: char = '-';
        const SIGN_MULTIPLY: char = '*';
        const SIGN_DIVIDE: char = '/';
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        let len_s: usize = s.len();
        let mut num: i32 = 0;
        let mut ans: i32 = 0;
        let mut sign: i32 = 1;
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_s);
        stk.push_back(sign);
        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    num = num * 10 + (ch as i32 - '0' as i32);
                }
                SIGN_PLUS | SIGN_MINUS => {
                    ans += sign * num;
                    sign = stk.back().unwrap() * if ch == SIGN_PLUS { 1 } else { -1 };
                    num = 0;
                }
                PAREN_OPEN => {
                    stk.push_back(sign);
                }
                PAREN_CLOSED => {
                    stk.pop_back();
                }
                _ => {}
            }
        }
        ans += sign * num;
        ans
    }
}
