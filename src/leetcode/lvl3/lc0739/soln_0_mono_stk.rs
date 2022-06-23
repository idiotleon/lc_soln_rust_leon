/// @author: Leon
/// https://leetcode.com/problems/daily-temperatures/
/// Time Complexity:    O(`len_t`)
/// Space Complexity:   O(`len_t`)
struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len_t = temperatures.len();
        let mut stk: VecDeque<usize> = VecDeque::new();
        let ans: Vec<i32> = {
            let mut tmp = vec![0; len_t];
            for (idx, &t) in temperatures.iter().enumerate() {
                while let Some(&idx_prev) = stk.back() {
                    if temperatures[idx_prev] < t {
                        tmp[idx_prev] = (idx - idx_prev) as i32;
                        stk.pop_back();
                    } else {
                        break;
                    }
                }
                stk.push_back(idx);
            }
            while let Some(idx) = stk.pop_back() {
                tmp[idx] = 0;
            }
            tmp
        };
        ans
    }
}
