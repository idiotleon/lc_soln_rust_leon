use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/basic-calculator-ii/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const SPACE: char = ' ';
    const SIGN_PLUS: char = '+';
    const SIGN_MINUS: char = '-';
    const SIGN_MULTIPLY: char = '*';
    const SIGN_DIVIDE: char = '/';
    pub fn calculate(s: String) -> i32 {
        let len_s = s.len();
        let mut stk: VecDeque<i32> = VecDeque::new();
        let mut num = 0;
        let mut op = Self::SIGN_PLUS;
        for (idx, ch) in s.chars().into_iter().enumerate() {
            if ch.is_digit(10) {
                num = num * 10 + (ch as i32 - '0' as i32);
            }
            if !ch.is_digit(10) && ch != Self::SPACE || idx == len_s - 1 {
                match op {
                    Self::SIGN_PLUS => stk.push_back(num),
                    Self::SIGN_MINUS => stk.push_back(-num),
                    Self::SIGN_MULTIPLY => {
                        let product = num * stk.pop_back().unwrap();
                        stk.push_back(product);
                    }
                    Self::SIGN_DIVIDE => {
                        let quotient = stk.pop_back().unwrap() / num;
                        stk.push_back(quotient);
                    }
                    _ => {}
                }
                op = ch;
                num = 0;
            }
        }
        let ans: i32 = {
            let mut ans: i32 = 0;
            while let Some(top) = stk.pop_front() {
                ans += top;
            }
            ans
        };
        ans
    }
}
