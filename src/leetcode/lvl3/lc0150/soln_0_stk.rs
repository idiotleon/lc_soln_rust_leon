/// @author: Leon
/// https://leetcode.com/problems/evaluate-reverse-polish-notation/
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
/// Reference:
/// https://leetcode.com/problems/evaluate-reverse-polish-notation/discuss/490442/Rust-simple-solution
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        for token in tokens.iter() {
            if let Ok(num) = token.parse() {
                stack.push_back(num);
            } else if let (Some(rhs), Some(lhs)) = (stack.pop_back(), stack.pop_back()) {
                stack.push_back(match token.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    _ => unreachable!(),
                })
            }
        }
        stack[0]
    }
}
